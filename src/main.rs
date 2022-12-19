#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nanos_sdk::testing::sdk_test_runner)]
#![reexport_test_harness_main = "test_main"]

mod crypto;
mod internal_ui;
mod transaction;
mod transactions;
mod utils;

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_sdk::io::StatusWords;
use nanos_ui::bagls::*;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};
use nanos_ui::screen_util;
use nanos_ui::ui;
use utils::{Context, SigningContext};

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// Show address confirmation
const P1_CONFIRM: u8 = 1u8;
// Don't show address confirmation
const P1_NON_CONFIRM: u8 = 0u8;
// Parameter 1 = End of Bytes to Sign (finalize)
const P1_LAST: u8 = 0x80;
// Parameter 1 = More bytes coming
const P1_MORE: u8 = 0x00;

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(ctx: &SigningContext) -> Result<([u8; 64], u32), StatusWords> {
    {
        match transactions::ask(ctx) {
            Ok(true) => {
                let signature = crypto::sign(ctx.buffer.as_bytes(), &ctx.bip32)?;
                Ok(signature)
            }
            Ok(false) => Err(StatusWords::UserCancelled),
            Err(_) => Err(StatusWords::Unknown),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    #[cfg(test)]
    test_main();

    // Exiting the application after completing tests
    #[cfg(test)]
    nanos_sdk::exit_app(0);

    let mut ctx = Context::new();

    // Number of displayed pages
    const PAGE_COUNT: usize = 3;
    // Current page displayed
    let mut cur_page = 0;

    let draw = |page: usize| {
        ui::clear_screen();

        match page {
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
    };

    draw(cur_page);

    loop {
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

                draw(cur_page);
            }
            io::Event::Button(ButtonEvent::RightButtonRelease) => {
                if cur_page + 1 < PAGE_COUNT {
                    cur_page += 1;
                } else {
                    cur_page = 0;
                }

                draw(cur_page);
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
            io::Event::Command(ins) => {
                match handle_apdu(&mut comm, ins, &mut ctx) {
                    Ok(()) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }

                draw(cur_page);
            }
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

fn handle_apdu(comm: &mut io::Comm, ins: Ins, ctx: &mut Context) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::Sign => {
            let data = comm.get_data()?;

            match comm.get_p1() {
                P1_MORE => {
                    // If this is a first chunk
                    if ctx.signing_context.buffer.length() == 0 {
                        // Then there're the bip32 path in the first chunk - first 20 bytes of data
                        let mut buf = [0u8; 20];
                        buf.clone_from_slice(&data[..20]);

                        let path = crypto::get_derivation_path(&mut &buf[..])?;
                        ctx.signing_context.bip32 = path;

                        // 21 byte - amount decimals
                        ctx.signing_context.amount_decimals = data[20];
                        // 22 byte - fee decimals
                        ctx.signing_context.fee_decimals = data[21];

                        ctx.signing_context.buffer.push(&data[22..]);
                    } else {
                        ctx.signing_context.buffer.push(data);
                    }
                }
                P1_LAST => {
                    ctx.signing_context.buffer.push(data);

                    let (signature_buf, length) = sign_ui(&ctx.signing_context)?;
                    comm.append(&signature_buf[..length as usize]);

                    ctx.signing_context.buffer.clean();
                }
                _ => {
                    ctx.signing_context.buffer.clean();
                    return Err(io::StatusWords::Unknown.into());
                }
            }
        }
        Ins::GetPubkey => {
            let mut data = comm.get_data()?;

            let path = crypto::get_derivation_path(&mut data)?;

            let public_key = crypto::get_pubkey(&path)?;

            let chain_id = comm.get_p2();

            let mut address = [0u8; 36];
            public_key
                .clone()
                .as_address(chain_id)
                .to_base58(&mut address);

            let mut result = [0u8; 67];
            result[..32].clone_from_slice(public_key.as_bytes());
            result[32..].clone_from_slice(&address[..35]);

            match comm.get_p1() {
                P1_CONFIRM => {
                    if internal_ui::verify_address(&mut address) {
                        comm.append(&result);
                    } else {
                        return Err(io::StatusWords::UserCancelled.into());
                    }
                }
                P1_NON_CONFIRM => {
                    comm.append(&result);
                }
                _ => return Err(io::StatusWords::Unknown.into()),
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
