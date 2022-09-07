use crate::transaction::transaction_data::TransactionData;
use crate::transaction::type_id::Type;
use crate::transaction::Transaction;
use crate::utils;
use core::str;

/// Maximum size of the message list
const MAX_SIZE: usize = 10;

/// Converts the transaction structure into messages for display on the screen
pub fn create<'a>(
    tx: Transaction,
    buf: &'a mut [u8],
) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize) {
    let mut titles = [""; MAX_SIZE];
    let mut messages = [""; MAX_SIZE];

    let mut cursor: usize = 0;

    // Temporary buffer for numtoa
    let mut buffer = [0u8; 20];

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

    // Get the formatted fee amount
    let (fee_bytes, fee_size) = utils::print_amount(tx.fee, &mut buffer);

    // TODO: all types and all fields
    match tx.data {
        TransactionData::Transfer { amount, asset, .. } => {
            // Get the formatted amount
            let (amount_bytes, amount_size) = utils::print_amount(amount, &mut buffer);

            // Transfer all amounts from the temp buffer to the total buffer
            buf[..amount_size].clone_from_slice(&amount_bytes[..amount_size]);
            buf[amount_size..amount_size + fee_size].clone_from_slice(&fee_bytes[..fee_size]);

            // Amount
            {
                let result = unsafe { str::from_utf8_unchecked(&buf[..amount_size]) };

                cursor += 1;
                titles[cursor - 1..cursor].clone_from_slice(&[&"Amount"]);
                messages[cursor - 1..cursor].clone_from_slice(&[&result]);
            }

            // Asset
            {
                if asset.is_none() {
                    cursor += 1;
                    titles[cursor - 1..cursor].clone_from_slice(&[&"Asset"]);
                    messages[cursor - 1..cursor].clone_from_slice(&[&"WEST"]);
                } else {
                    // TODO: Display asset hash
                    cursor += 1;
                    titles[cursor - 1..cursor].clone_from_slice(&[&"Asset"]);
                    messages[cursor - 1..cursor].clone_from_slice(&[&"None"]);
                }
            }

            // Fee
            {
                let result =
                    unsafe { str::from_utf8_unchecked(&buf[amount_size..amount_size + fee_size]) };

                cursor += 1;
                titles[cursor - 1..cursor].clone_from_slice(&[&"Fee"]);
                messages[cursor - 1..cursor].clone_from_slice(&[&result]);
            }
        }
        _ => (),
    }

    (titles, messages, cursor)
}
