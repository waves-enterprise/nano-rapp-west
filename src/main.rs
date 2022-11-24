#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nanos_sdk::sdk_test_runner)]
#![reexport_test_harness_main = "test_main"]

mod internal_ui;
mod sodium;
mod transaction;
mod transactions;
mod utils;

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::ecc::Ed25519;
use nanos_sdk::io;
use nanos_sdk::io::SyscallError;
use nanos_ui::bagls::*;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};
use nanos_ui::screen_util;
use nanos_ui::ui;
use transaction::account::PublicKeyAccount;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(message: &[u8]) -> Result<Option<([u8; 64], u32)>, SyscallError> {
    {
        match transactions::ask(message) {
            Ok(true) => {
                let signature = Ed25519::new()
                    .sign(message)
                    .map_err(|_| SyscallError::Unspecified)?;
                Ok(Some(signature))
            }
            Ok(false) => Ok(None),
            Err(_) => Ok(None),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    #[cfg(test)]
    test_main();

    // Number of displayed pages
    const PAGE_COUNT: usize = 3;
    // Current page displayed
    let mut cur_page = 0;

    loop {
        ui::clear_screen();

        match cur_page {
            0 => {
                ["Application", "is ready"].place(Location::Middle, Layout::Centered, false);
            }
            1 => {
                let version = env!("CARGO_PKG_VERSION");

                "Version".place(Location::Top, Layout::Centered, true);
                version.place(Location::Middle, Layout::Centered, false);
            }
            2 => {
                "Quit".place(Location::Middle, Layout::Centered, true);
            }
            _ => (),
        }

        LEFT_ARROW.display();
        RIGHT_ARROW.display();

        screen_util::screen_update();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::LeftButtonPress) => {
                LEFT_S_ARROW.instant_display();
            }
            io::Event::Button(ButtonEvent::RightButtonPress) => {
                RIGHT_S_ARROW.instant_display();
            }
            io::Event::Button(ButtonEvent::LeftButtonRelease) => {
                if cur_page > 0 {
                    cur_page -= 1;
                } else {
                    cur_page = PAGE_COUNT - 1;
                }
            }
            io::Event::Button(ButtonEvent::RightButtonRelease) => {
                if cur_page + 1 < PAGE_COUNT {
                    cur_page += 1;
                } else {
                    cur_page = 0;
                }
            }
            io::Event::Button(ButtonEvent::BothButtonsRelease) => {
                // Selecting a menu item
                match cur_page {
                    0 => (),
                    1 => (),
                    2 => nanos_sdk::exit_app(0),
                    _ => (),
                }
            }
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
    GetVersion,
    GetName,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            2 => Ins::Sign,
            4 => Ins::GetPubkey,
            6 => Ins::GetVersion,
            8 => Ins::GetName,
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
        Ins::Sign => {
            let out = sign_ui(comm.get_data()?)?;
            if let Some((signature_buf, length)) = out {
                comm.append(&signature_buf[..length as usize])
            }
        }
        Ins::GetPubkey => {
            let pk = Ed25519::new()
                .public_key()
                .map_err(|x| Reply(0x6eu16 | (x as u16 & 0xff)))?;
            let pk_be = PublicKeyAccount::from_ed25519(pk.as_ref());

            let chain_id = comm.get_p2();

            let mut address = [0u8; 36];
            pk_be.clone().to_address(chain_id).to_base58(&mut address);

            let mut result = [0u8; 67];
            result[..32].clone_from_slice(pk_be.to_bytes());
            result[32..].clone_from_slice(&address[..35]);

            if comm.get_p1() == 1u8 {
                if internal_ui::verify_address(&mut address) {
                    comm.append(&result);
                } else {
                    return Err(io::StatusWords::UserCancelled.into());
                }
            } else {
                comm.append(&result);
            }
        }
        Ins::GetVersion => {
            let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
            let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
            comm.append([version_major, version_minor, version_patch].as_slice())
        }
        Ins::GetName => comm.append(b"Waves Enterprise"),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
