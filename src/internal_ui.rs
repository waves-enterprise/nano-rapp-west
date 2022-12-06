mod horizontal_validator;

use core::str;

use crate::{single_screen, three_screens};

pub use horizontal_validator::*;

const MAX_SIZE: usize = 4;

pub fn verify_address(address: &mut [u8]) -> bool {
    let mut titles = [""; MAX_SIZE];
    let mut messages = [""; MAX_SIZE];
    let mut cursor: usize = 0;

    single_screen!("Verify", "address", cursor, titles, messages);

    three_screens!("Address", address, cursor, titles, messages);

    HorizontalValidator::new(&titles, &messages, TypeValidator::Verify).ask()
}
