pub const HASH_LENGTH: usize = 32;

pub type TransactionId = Hash;
pub type Asset = Hash;

pub struct Hash([u8; HASH_LENGTH]);

/// Representation of transaction hash, asset and other
impl Hash {
    pub fn new(bytes: [u8; HASH_LENGTH]) -> Self {
        Hash(bytes)
    }

    pub fn is_empty(&self) -> bool {
        let mut length = self.0.len();

        while length > 0 {
            let item = self.0[length - 1];

            if item != 0u8 {
                return false;
            }

            length -= 1;
        }

        true
    }

    pub fn to_bytes(&self) -> [u8; HASH_LENGTH] {
        self.0
    }

    /// Converts to an Asset
    /// Some(self) - any other Asset
    /// None - WEST
    pub fn to_asset(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(Hash::new(self.0))
        }
    }
}
