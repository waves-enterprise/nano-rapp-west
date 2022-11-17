use crate::transaction::account::{PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::utils;

use core::str;

use crate::transactions::*;
use crate::{convert_numbers, single_screen};

#[allow(dead_code)]
pub struct CreateContract {
    type_id: Type,
    version: Version,
    sender_public_key: PublicKeyAccount,
    // image: &[u8],
    // image_hash: Hash,
    // contract_name: &[u8],
    fee: u64,
    timestamp: u64,
}

impl<'a> Transaction<'a> for CreateContract {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        // let mut image = [0u8; 600];
        // let mut image_hash = [0u8; HASH_LENGTH];
        // let mut contract_name = [0u8; 600];
        let mut fee = [0u8; 8];
        let mut timestamp = [0u8; 8];

        buffer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .skip_string() // TODO: .get_string(&mut image)
            .skip_string() // TODO: .get_string(&mut image_hash)
            .skip_string() // TODO: .get_string(&mut contract_name)
            .skip_params()
            .get_bytes(&mut fee, 8)
            .get_bytes(&mut timestamp, 8);

        CreateContract {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            sender_public_key: PublicKeyAccount::new(sender_public_key),
            // image,
            // image_hash,
            // contract_name,
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
        single_screen!("Review", "creating contract", cursor, titles, messages);

        // Fee
        single_screen!("Fee", fee, cursor, titles, messages);

        // Fee asset
        single_screen!("Fee asset", "WEST", cursor, titles, messages);

        (titles, messages, cursor)
    }
}
