use crate::transaction::transaction_data::*;
use crate::transaction::Transaction;
use core::char;
use numtoa::NumToA;

/// Maximum size of the message list
const MAX_SIZE: usize = 10;

/// Converts the transaction structure into messages for display on the screen
pub fn create_messages<'a>(
    tx: Transaction,
    buf: &'a mut [u8],
) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE]) {
    let mut titles = [""; MAX_SIZE];
    let mut messages = [""; MAX_SIZE];

    let mut cursor: usize = 0;

    // Forms a message about the type of transaction
    {
        cursor += 1;

        titles[cursor - 1..cursor].clone_from_slice(&[&"Review"]);

        // TODO: all types
        match tx.type_id {
            Type::Transfer => {
                messages[cursor - 1..cursor].clone_from_slice(&[&"transfer"]);
            }
            _ => {
                messages[cursor - 1..cursor].clone_from_slice(&[&"unknown tx"]);
            }
        };
    }

    // TODO: all types and all fields
    match tx.data {
        TransactionData::Transfer { amount, asset, .. } => {
            {
                let result = amount.numtoa_str(10, buf);

                cursor += 1;
                titles[cursor - 1..cursor].clone_from_slice(&[&"Amount"]);
                messages[cursor - 1..cursor].clone_from_slice(&[&result]);
            }

            {
                cursor += 1;
                titles[cursor - 1..cursor].clone_from_slice(&[&"Asset"]);
                messages[cursor - 1..cursor].clone_from_slice(&[&"WEST"]);
            }
        }
        _ => (),
    }

    (titles, messages)
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
