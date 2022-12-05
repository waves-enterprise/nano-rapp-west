mod burn;
mod create_contract;
mod issue;
mod reissue;
mod transfer;

use burn::Burn;
use create_contract::CreateContract;
use issue::Issue;
use reissue::Reissue;
use transfer::Transfer;

use crate::transaction::type_id::Type;
use crate::transaction::version::Version;
use crate::utils::deserialize::Buffer;

/// Maximum size of the message list
pub const MAX_SIZE: usize = 15;

pub enum TransactionError {
    IncorrectTransaction,
}

trait Transaction<'a> {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn ask(&self) -> bool;
}

pub fn ask<'a>(message: &[u8]) -> Result<bool, TransactionError> {
    let mut buffer = Buffer::new(message);

    let mut type_id = 0_u8;
    let mut version = 0_u8;

    buffer.get_byte(&mut type_id).get_byte(&mut version);

    match (Type::from_u8(type_id), Version::from_u8(version)) {
        (Type::Issue, Version::V2) => Ok(Issue::from_bytes(message).ask()),
        (Type::Transfer, Version::V2) => Ok(Transfer::from_bytes(message).ask()),
        (Type::Reissue, Version::V2) => Ok(Reissue::from_bytes(message).ask()),
        (Type::Burn, Version::V2) => Ok(Burn::from_bytes(message).ask()),
        (Type::CreateContract, Version::V2) => Ok(CreateContract::from_bytes(message).ask()),
        _ => Err(TransactionError::IncorrectTransaction),
    }
}
