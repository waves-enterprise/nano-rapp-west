use crate::transaction::account::{Address, PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::transaction::hash::{Asset, Hash, HASH_LENGTH};
use crate::transaction::type_id::Type;
use crate::transaction::version::Version;
use crate::utils;
use crate::utils::deserialize::Buffer;

use core::str;

use crate::transactions::*;

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

        // Temporary buffer for numtoa
        let mut buffer = [0u8; 20];

        {
            cursor += 1;

            titles[cursor - 1..cursor].clone_from_slice(&[&"Review"]);
            messages[cursor - 1..cursor].clone_from_slice(&[&"transfer"]);
        }

        // Get the formatted fee amount
        let (fee_bytes, fee_size) = utils::print_amount(self.fee, &mut buffer);

        let (amount_bytes, amount_size) = utils::print_amount(self.amount, &mut buffer);

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
            if self.asset.is_none() {
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

        (titles, messages, cursor)
    }
}
