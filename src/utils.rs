mod context;
mod deserialize;

pub mod macros;

use numtoa::NumToA;

pub use context::*;
pub use deserialize::*;

/// Converting numbers to a formatted bytes
pub fn number_to_formatted_bytes(number: u64, buf: &mut [u8], decimals: u8) -> ([u8; 20], usize) {
    let mut buffer = [0u8; 20];
    #[allow(unused_assignments)]
    let mut cursor = 0;

    let quotient = number.div_euclid(10u64.saturating_pow(decimals as u32));
    let reste = number.rem_euclid(10u64.saturating_pow(decimals as u32));

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

#[cfg(test)]
mod tests {
    use super::*;

    use core::str;
    use nanos_sdk::testing::TestType;

    fn num_to_str() -> Result<(), ()> {
        let mut temp = [0u8; 20];

        let (buf, buf_size) = number_to_formatted_bytes(100, &mut temp, 2);

        let result = unsafe { str::from_utf8_unchecked(&buf[..buf_size]) };

        if "1.000".eq_ignore_ascii_case(result) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_account_address: TestType = TestType {
        modname: "utils",
        name: "num_to_str",
        f: num_to_str,
    };
}
