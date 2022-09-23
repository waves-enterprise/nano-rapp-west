use crate::transaction::account::{Address, PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::transaction::hash::{Asset, Hash, HASH_LENGTH};
use crate::transaction::type_id::Type;
use crate::transaction::version::Version;
use crate::utils;
use crate::utils::deserialize::Buffer;

use core::str;

use crate::transactions::*;
use crate::{convert_numbers, hash_screen, single_screen};

pub struct Transfer<'a> {
    type_id: Type,
    version: Version,
    sender_public_key: PublicKeyAccount,
    asset: Option<Asset>,
    fee_asset: Option<Asset>,
    timestamp: u64,
    amount: u64,
    fee: u64,
    recipient: Address,
    attachment: Option<&'a str>,
}

impl<'a> Transaction<'a> for Transfer<'a> {
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
            attachment: None, // TODO: Parse attachment
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
