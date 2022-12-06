use crate::internal_ui::{HorizontalValidator, TypeValidator};
use crate::transaction::account::{PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::utils::number_to_formatted_bytes;

use core::str;

use crate::transactions::*;
use crate::{convert_number_to_str, impl_transactions_test, single_screen};

#[allow(dead_code)]
pub struct Issue {
    type_id: Type,
    version: Version,
    chain_id: u8,
    sender_public_key: PublicKeyAccount,
    quantity: u64,
    decimals: u8,
    reissuable: bool,
    fee: u64,
    timestamp: u64,
}

impl<'a> Transaction<'a> for Issue {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut chain_id = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        let mut quantity = [0u8; 8];
        let mut decimals = 0_u8;
        let mut reissuable = false;
        let mut fee = [0u8; 8];
        let mut timestamp = [0u8; 8];

        buffer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_byte(&mut chain_id)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .skip_string()
            .skip_string()
            .get_bytes(&mut quantity, 8)
            .get_byte(&mut decimals)
            .get_bool(&mut reissuable)
            .get_bytes(&mut fee, 8)
            .get_bytes(&mut timestamp, 8);

        Issue {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            chain_id,
            sender_public_key: PublicKeyAccount::new(sender_public_key),
            quantity: u64::from_be_bytes(quantity),
            decimals,
            reissuable,
            fee: u64::from_be_bytes(fee),
            timestamp: u64::from_be_bytes(timestamp),
        }
    }

    fn ask(&self) -> bool {
        let mut titles = [""; MAX_SIZE];
        let mut messages = [""; MAX_SIZE];
        let mut cursor: usize = 0;

        // Temporary buffer to convert number to string
        let mut temp = [0u8; 20];

        // Transaction type
        single_screen!("Review", "issue", cursor, titles, messages);

        // Fee
        let fee: &str;
        convert_number_to_str!(self.fee, fee, temp);
        single_screen!("Fee", fee, cursor, titles, messages);

        // Fee asset
        single_screen!("Fee asset", "WEST", cursor, titles, messages);

        // Run the show and get an answer
        HorizontalValidator::new(&titles[..cursor], &messages[..cursor], TypeValidator::Sign).ask()
    }
}

// Example transaction for tests
#[allow(dead_code)]
const BYTES: [u8; 74] = [
    3, 2, 87, 30, 179, 95, 61, 75, 82, 107, 179, 157, 154, 213, 160, 129, 207, 205, 75, 153, 37,
    53, 128, 108, 244, 145, 136, 134, 145, 43, 17, 46, 65, 200, 8, 0, 4, 84, 101, 115, 116, 0, 4,
    116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 1, 116, 16,
    180, 2, 72, 0,
];

impl_transactions_test!(Issue, Type::Issue, Version::V2, 100000000);
