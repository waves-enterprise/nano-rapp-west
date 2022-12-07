use crate::internal_ui::{HorizontalValidator, TypeValidator};
use crate::transaction::account::{PublicKeyAccount, PUBLIC_KEY_LENGTH};
use crate::utils::number_to_formatted_bytes;

use core::str;

use crate::transactions::*;
use crate::{convert_number_to_str, single_screen};

#[allow(dead_code)]
pub struct CreateContract {
    type_id: Type,
    version: Version,
    sender_public_key: PublicKeyAccount,
    fee: u64,
    timestamp: u64,
}

impl<'a> Transaction<'a> for CreateContract {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut deserializer = Deserializer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;
        let mut sender_public_key = [0u8; PUBLIC_KEY_LENGTH];
        let mut fee = [0u8; 8];
        let mut timestamp = [0u8; 8];

        deserializer
            .get_byte(&mut type_id)
            .get_byte(&mut version)
            .get_bytes(&mut sender_public_key, PUBLIC_KEY_LENGTH)
            .skip_string()
            .skip_string()
            .skip_string()
            .skip_params()
            .get_bytes(&mut fee, 8)
            .get_bytes(&mut timestamp, 8);

        CreateContract {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
            sender_public_key: PublicKeyAccount::new(sender_public_key),
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
        single_screen!("Review", "creating contract", cursor, titles, messages);

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
