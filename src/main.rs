#![no_std]
#![no_main]

mod sodium;
mod transaction;
mod transactions;
mod utils;

use core::str::from_utf8;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::ecc::Ed25519;
use nanos_sdk::io;
use nanos_sdk::io::SyscallError;
use nanos_ui::ui;
use transaction::account::PublicKeyAccount;
use utils::tx_scroller::TxScroller;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

/// Display public key in two separate
/// message scrollers
fn show_pubkey() {
    let pubkey = Ed25519::new().public_key();
    match pubkey {
        Ok(value) => {
            let pubkey_be = PublicKeyAccount::from_ed25519(value.as_ref());

            let hex = utils::to_hex(pubkey_be.to_bytes()).unwrap();
            let m = from_utf8(&hex).unwrap();
            ui::MessageScroller::new(m).event_loop();
        }
        Err(_) => ui::popup("Error"),
    }
}

/// Basic nested menu. Will be subject
/// to simplifications in the future.
#[allow(clippy::needless_borrow)]
fn menu_example() {
    loop {
        match ui::Menu::new(&[&"PubKey", &"Infos", &"Back", &"Exit App"]).show() {
            0 => show_pubkey(),
            1 => loop {
                match ui::Menu::new(&[&"Copyright", &"Authors", &"Back"]).show() {
                    0 => ui::popup("2020 Ledger"),
                    1 => ui::popup("???"),
                    _ => break,
                }
            },
            2 => return,
            3 => nanos_sdk::exit_app(0),
            _ => (),
        }
    }
}

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(message: &[u8]) -> Result<Option<([u8; 64], u32)>, SyscallError> {
    {
        // This buffer is intended for storing all amounts
        // and should live all the time until everything is displayed
        let mut buf = [0u8; 60];
        match transactions::create_messages_from_bytes(message, &mut buf) {
            Ok((titles, messages, length)) => {
                TxScroller::new(&titles[..length], &messages[..length]).event_loop()
            }
            Err(_err) => {
                ui::popup("Invalid transaction");
                return Ok(None);
            }
        }
    }

    if ui::Validator::new("Sign ?").ask() {
        let signature = Ed25519::new()
            .sign(message)
            .map_err(|_| SyscallError::Unspecified)?;
        ui::popup("Done !");
        Ok(Some(signature))
    } else {
        ui::popup("Cancelled");
        Ok(None)
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        // Draw some 'welcome' screen
        ui::SingleMessage::new("W e l c o m e").show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            io::Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetPubkey,
    Sign,
    Menu,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            2 => Ins::GetPubkey,
            3 => Ins::Sign,
            4 => Ins::Menu,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetPubkey => {
            let pk = Ed25519::new()
                .public_key()
                .map_err(|x| Reply(0x6eu16 | (x as u16 & 0xff)))?;
            let pk_be = PublicKeyAccount::from_ed25519(pk.as_ref());
            comm.append(pk_be.to_bytes());
        }
        Ins::Sign => {
            let out = sign_ui(comm.get_data()?)?;
            if let Some((signature_buf, length)) = out {
                comm.append(&signature_buf[..length as usize])
            }
        }
        Ins::Menu => menu_example(),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
