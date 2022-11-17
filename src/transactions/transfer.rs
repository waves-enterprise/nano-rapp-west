use crate::transaction::account::{Address, PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::transaction::hash::{Asset, Hash, HASH_LENGTH};
use crate::utils;

use core::str;

use crate::transactions::*;
use crate::{convert_numbers, hash_screen, impl_simple_test, single_screen};

#[allow(dead_code)]
pub struct Transfer {
    type_id: Type,
    version: Version,
    sender_public_key: PublicKeyAccount,
    asset: Option<Asset>,
    fee_asset: Option<Asset>,
    timestamp: u64,
    amount: u64,
    fee: u64,
    recipient: Address,
}

impl<'a> Transaction<'a> for Transfer {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        let mut asset_bytes = [0u8; HASH_LENGTH];
        let mut fee_asset_bytes = [0u8; HASH_LENGTH];
        let mut timestamp = [0u8; 8];
        let mut amount = [0u8; 8];
        let mut fee = [0u8; 8];
        let mut recipient: [u8; 26] = [0u8; 26];

        buffer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .get_bytes_flag(&mut asset_bytes, HASH_LENGTH)
            .get_bytes_flag(&mut fee_asset_bytes, HASH_LENGTH)
            .get_bytes(&mut timestamp, 8)
            .get_bytes(&mut amount, 8)
            .get_bytes(&mut fee, 8);
        // TODO: Parse recipient

        let asset = Hash::new(asset_bytes).to_asset();
        let fee_asset = Hash::new(fee_asset_bytes).to_asset();

        Transfer {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            sender_public_key: PublicKeyAccount::new(sender_public_key),
            asset,
            fee_asset,
            timestamp: u64::from_be_bytes(timestamp),
            amount: u64::from_be_bytes(amount),
            fee: u64::from_be_bytes(fee),
            recipient: Address::new(recipient),
        }
    }

    fn to_messages(&self, buf: &'a mut [u8]) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize) {
        let mut titles = [""; MAX_SIZE];
        let mut messages = [""; MAX_SIZE];

        let mut cursor: usize = 0;

        // Convert all the numbers
        let amount: &str;
        let fee: &str;
        convert_numbers!([self.amount, self.fee], [amount, fee], buf);

        // Name tx
        single_screen!("Review", "transfer", cursor, titles, messages);

        // Amount
        single_screen!("Amount", amount, cursor, titles, messages);

        // Asset
        hash_screen!("Asset", &self.asset, cursor, titles, messages);

        // Fee
        single_screen!("Fee", fee, cursor, titles, messages);

        // Fee asset
        hash_screen!("Fee asset", &self.fee_asset, cursor, titles, messages);

        (titles, messages, cursor)
    }
}

// Example transaction for tests
#[allow(dead_code)]
const BYTES: [u8; 141] = [
    4, 2, 30, 179, 95, 61, 75, 82, 107, 179, 157, 154, 213, 160, 129, 207, 205, 75, 153, 37, 53,
    128, 108, 244, 145, 136, 134, 145, 43, 17, 46, 65, 200, 8, 0, 0, 0, 0, 1, 116, 16, 180, 2, 72,
    0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 0, 0, 0, 15, 66, 64, 1, 86, 64, 178, 202, 112, 130, 11, 170,
    59, 133, 11, 247, 67, 236, 108, 82, 199, 157, 226, 40, 227, 255, 5, 251, 149, 0, 53, 2, 177,
    218, 94, 250, 30, 209, 137, 196, 245, 194, 30, 23, 37, 110, 45, 233, 145, 134, 180, 44, 180,
    125, 63, 125, 60, 183, 50, 1, 88, 109, 231, 132, 235, 246, 250, 38, 154, 127, 34, 104, 204,
    206, 90, 191, 69, 182, 4, 4, 120, 236, 31, 54,
];

impl_simple_test!(Transfer, Type::Transfer, Version::V2, 1000000);
