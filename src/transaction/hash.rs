pub const HASH_LENGTH: usize = 32;

pub type TransactionId = Hash;
pub type Asset = Hash;

pub struct Hash([u8; HASH_LENGTH]);

impl Hash {
    pub fn new(bytes: [u8; HASH_LENGTH]) -> Hash {
        Hash(bytes)
    }

    pub fn to_bytes(&self) -> [u8; HASH_LENGTH] {
        self.0
    }
}
