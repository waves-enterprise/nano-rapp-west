use crate::transaction::account::{PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::transaction::hash::{Asset, Hash, HASH_LENGTH};
use crate::utils;

use core::str;

use crate::transactions::*;
use crate::{convert_numbers, impl_transactions_test, single_screen};

#[allow(dead_code)]
pub struct Reissue {
    type_id: Type,
    version: Version,
    chain_id: u8,
    sender_public_key: PublicKeyAccount,
    asset_id: Asset,
    quantity: u64,
    reissuable: bool,
    fee: u64,
    timestamp: u64,
}

impl<'a> Transaction<'a> for Reissue {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut chain_id = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        let mut asset_bytes = [0u8; HASH_LENGTH];
        let mut quantity = [0u8; 8];
        let mut reissuable = false;
        let mut fee = [0u8; 8];
        let mut timestamp = [0u8; 8];

        buffer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_byte(&mut chain_id)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .get_bytes(&mut asset_bytes, HASH_LENGTH)
            .get_bytes(&mut quantity, 8)
            .get_bool(&mut reissuable)
            .get_bytes(&mut fee, 8)
            .get_bytes(&mut timestamp, 8);

        let asset_id = Hash::new(asset_bytes);

        Reissue {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            chain_id,
            sender_public_key: PublicKeyAccount::new(sender_public_key),
            asset_id,
            quantity: u64::from_be_bytes(quantity),
            reissuable,
            fee: u64::from_be_bytes(fee),
            timestamp: u64::from_be_bytes(timestamp),
        }
    }

    fn to_messages(&self, buf: &'a mut [u8]) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize) {
        let mut titles = [""; MAX_SIZE];
        let mut messages = [""; MAX_SIZE];

        let mut cursor: usize = 0;

        // Convert all the numbers
        let fee: &str;
        convert_numbers!([self.fee], [fee], buf);

        // Name tx
        single_screen!("Review", "reissue", cursor, titles, messages);

        // Fee
        single_screen!("Fee", fee, cursor, titles, messages);

        // Fee asset
        single_screen!("Fee asset", "WEST", cursor, titles, messages);

        (titles, messages, cursor)
    }
}

// Example transaction for tests
#[allow(dead_code)]
const BYTES: [u8; 92] = [
    5, 2, 87, 30, 179, 95, 61, 75, 82, 107, 179, 157, 154, 213, 160, 129, 207, 205, 75, 153, 37,
    53, 128, 108, 244, 145, 136, 134, 145, 43, 17, 46, 65, 200, 8, 30, 179, 95, 61, 75, 82, 107,
    179, 157, 154, 213, 160, 129, 207, 205, 75, 153, 37, 53, 128, 108, 244, 145, 136, 134, 145, 43,
    17, 46, 65, 200, 8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 1, 116, 16,
    180, 2, 72,
];

impl_transactions_test!(Reissue, Type::Reissue, Version::V2, 100000000);
