mod data_buffer;
mod deserialize;

pub mod crypto;
pub mod macros;

use numtoa::NumToA;

pub use data_buffer::*;
pub use deserialize::*;

const DECIMALS: u64 = 100000000;

/// Converting numbers to a formatted bytes
pub fn number_to_formatted_bytes(number: u64, buf: &mut [u8]) -> ([u8; 20], usize) {
    let mut buffer = [0u8; 20];
    #[allow(unused_assignments)]
    let mut cursor = 0;

    let quotient = number.div_euclid(DECIMALS);
    let reste = number.rem_euclid(DECIMALS);

    let quotient_str = quotient.numtoa_str(10, buf);
    cursor = quotient_str.as_bytes().len();
    buffer[..cursor].clone_from_slice(quotient_str.as_bytes());

    buffer[cursor..cursor + 1].clone_from_slice(b".");

    cursor += 1;

    let reste_str = reste.numtoa_str(10, buf);
    if reste_str.len() < 6 {
        buffer[cursor..cursor + 3].clone_from_slice(b"000");
    } else if reste_str.len() < 7 {
        buffer[cursor..cursor + 2].clone_from_slice(b"00");
        buffer[cursor + 2..cursor + 3].clone_from_slice(reste_str.split_at(1).0.as_bytes());
    } else if reste_str.len() < 8 {
        buffer[cursor..cursor + 1].clone_from_slice(b"0");
        buffer[cursor + 1..cursor + 3].clone_from_slice(reste_str.split_at(2).0.as_bytes());
    } else {
        buffer[cursor..cursor + 3].clone_from_slice(reste_str.split_at(3).0.as_bytes());
    }

    (buffer, cursor + 3)
}
