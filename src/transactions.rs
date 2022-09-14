pub mod transfer;

use transfer::Transfer;

use crate::transaction::type_id::Type;
use crate::transaction::version::Version;
use crate::utils::deserialize::Buffer;

/// Maximum size of the message list
pub const MAX_SIZE: usize = 10;

pub enum TransactionError {
    IncorrectTransaction,
}

trait Transaction<'a> {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_messages(&self, buf: &'a mut [u8]) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize);
}

pub fn create_messages_from_bytes<'a>(
    message: &[u8],
    buf: &'a mut [u8],
) -> Result<([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize), TransactionError> {
    let mut buffer = Buffer::new(message);

    let mut type_id = 0_u8;
    let mut version = 0_u8;

    buffer.get_byte(&mut type_id).get_byte(&mut version);

    match (Type::from_u8(type_id), Version::from_u8(version)) {
        (Type::Transfer, Version::V2) => Ok(Transfer::from_bytes(message).to_messages(buf)),
        _ => Err(TransactionError::IncorrectTransaction),
    }
}
