use crate::utils::horizontal_validator::{HorizontalValidator, TypeValidator};

use core::str;

use crate::{address_screen, single_screen};

const MAX_SIZE: usize = 4;

pub fn verify_address(address: &mut [u8]) -> bool {
    let mut titles = [""; MAX_SIZE];
    let mut messages = [""; MAX_SIZE];
    let mut cursor: usize = 0;

    single_screen!("Verify", "address", cursor, titles, messages);

    address_screen!(address, cursor, titles, messages);

    HorizontalValidator::new(&titles, &messages, TypeValidator::Verify).ask()
}
