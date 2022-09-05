use crate::transaction::transaction_data::*;
use crate::transaction::type_id::Type;
use crate::transaction::Transaction;
use crate::utils::print_amount;

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
                let result = print_amount(amount, buf);

                cursor += 1;
                titles[cursor - 1..cursor].clone_from_slice(&[&"Amount"]);
                messages[cursor - 1..cursor].clone_from_slice(&[&result]);
            }

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
        }
        _ => (),
    }

    (titles, messages, cursor)
}
