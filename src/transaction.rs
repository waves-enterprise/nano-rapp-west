mod account;
mod deserialize;
mod hash;
pub mod transaction_data;
pub mod type_id;

use account::{Address, PublicKeyAccount};
use deserialize::Buffer;
use hash::Hash;
use transaction_data::TransactionData;
use type_id::Type;

/// Transaction deserialization error types.
pub enum TransactionError {
    IncorrectType,
}

/// Transaction data. Data specific to a particular transaction type are stored in the `data` field.
pub struct Transaction<'a> {
    pub data: TransactionData<'a>,
    pub fee: u64,
    pub timestamp: u64,
    pub sender_public_key: PublicKeyAccount,
    pub type_id: Type,
    pub version: u8,
}

impl<'a> Transaction<'a> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Transaction<'a>, TransactionError> {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;

        let mut temp = buffer.get_byte(&mut type_id);
        let mut new_buffer = temp.get_byte(&mut version);

        // Transfer
        if type_id == 4u8 {
            let mut sender_public_key = [0u8; account::PUBLIC_KEY_LENGTH];
            let mut asset_bytes = [0u8; hash::HASH_LENGTH];
            let mut fee_asset_bytes = [0u8; hash::HASH_LENGTH];
            let mut timestamp = [0u8; 8];
            let mut amount = [0u8; 8];
            let mut fee = [0u8; 8];
            let mut recipient: [u8; 26] = [0u8; 26];

            new_buffer
                .get_bytes(&mut sender_public_key, account::PUBLIC_KEY_LENGTH)
                .get_bytes_flag(&mut asset_bytes, hash::HASH_LENGTH)
                .get_bytes_flag(&mut fee_asset_bytes, hash::HASH_LENGTH)
                .get_bytes(&mut timestamp, 8)
                .get_bytes(&mut amount, 8)
                .get_bytes(&mut fee, 8);
            // TODO: Parse recipient

            let asset = Hash::new(asset_bytes).to_asset();
            let fee_asset = Hash::new(fee_asset_bytes).to_asset();

            Ok(Transaction {
                data: TransactionData::Transfer {
                    recipient: Address::new(recipient),
                    asset,
                    amount: u64::from_be_bytes(amount),
                    fee_asset,
                    attachment: None, // TODO: Parse attachment
                },
                fee: u64::from_be_bytes(fee),
                timestamp: u64::from_be_bytes(timestamp),
                sender_public_key: PublicKeyAccount::new(sender_public_key),
                type_id: Type::from_u8(type_id),
                version,
            })
        } else {
            Err(TransactionError::IncorrectType)
        }
    }
}
