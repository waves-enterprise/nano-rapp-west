use crate::transaction::account::{PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::transaction::hash::{Asset, Hash, HASH_LENGTH};
use crate::utils;

use core::str;

use crate::transactions::*;
use crate::{convert_numbers, impl_simple_test, single_screen};

pub struct Burn {
    type_id: Type,
    version: Version,
    chain_id: u8,
    sender_public_key: PublicKeyAccount,
    asset_id: Asset,
    amount: u64,
    fee: u64,
    timestamp: u64,
}

impl<'a> Transaction<'a> for Burn {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut chain_id = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        let mut asset_bytes = [0u8; HASH_LENGTH];
        let mut amount = [0u8; 8];
        let mut fee = [0u8; 8];
        let mut timestamp = [0u8; 8];

        buffer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_byte(&mut chain_id)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .get_bytes(&mut asset_bytes, HASH_LENGTH)
            .get_bytes(&mut amount, 8)
            .get_bytes(&mut fee, 8)
            .get_bytes(&mut timestamp, 8);

        let asset_id = Hash::new(asset_bytes);

        Burn {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            chain_id,
            sender_public_key: PublicKeyAccount::new(sender_public_key),
            asset_id,
            amount: u64::from_be_bytes(amount),
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

        single_screen!("Review", "burn", cursor, titles, messages);

        // Fee
        single_screen!("Fee", fee, cursor, titles, messages);

        // Fee asset
        single_screen!("Fee asset", "WEST", cursor, titles, messages);

        (titles, messages, cursor)
    }
}

const BYTES: [u8; 2] = [6, 2];
impl_simple_test!(Burn, Type::Burn, Version::V2);
