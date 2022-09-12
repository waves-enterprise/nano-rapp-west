pub mod deserialize;
pub mod tx_scroller;

use core::char;
use numtoa::NumToA;

const DECIMALS: u64 = 100000000;

/// Amount formatting
pub fn print_amount<'a>(number: u64, buf: &'a mut [u8]) -> ([u8; 20], usize) {
    let mut buffer = [0u8; 20];
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

/// Convert to hex. Returns a static buffer of 64 bytes
#[inline]
pub fn to_hex(m: &[u8]) -> Result<[u8; 64], ()> {
    if 2 * m.len() > 64 {
        return Err(());
    }
    let mut hex = [0u8; 64];
    let mut i = 0;
    for c in m {
        let c0 = char::from_digit((c >> 4).into(), 16).unwrap();
        let c1 = char::from_digit((c & 0xf).into(), 16).unwrap();
        hex[i] = c0 as u8;
        hex[i + 1] = c1 as u8;
        i += 2;
    }
    Ok(hex)
}
