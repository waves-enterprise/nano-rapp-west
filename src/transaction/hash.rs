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

#[cfg(test)]
mod tests {
    use super::*;

    use nanos_sdk::TestType;

    fn hash_zero() -> Result<(), ()> {
        let bytes = [0u8; 32];
        let hash = Hash::new(bytes);

        if bytes == hash.to_bytes() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn hash_zero_is_empty() -> Result<(), ()> {
        let zero_bytes = [0u8; 32];
        let hash = Hash::new(zero_bytes);

        if hash.is_empty() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn hash_one_is_empty() -> Result<(), ()> {
        let one_bytes = [1u8; 32];
        let hash = Hash::new(one_bytes);

        if !hash.is_empty() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn hash_zero_asset() -> Result<(), ()> {
        let west_bytes = [0u8; 32];
        let hash = Hash::new(west_bytes);

        if hash.to_asset().is_none() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn hash_one_asset() -> Result<(), ()> {
        let asset_bytes = [1u8; 32];
        let hash = Hash::new(asset_bytes);

        if hash.to_asset().is_some() {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_hash_zero: TestType = TestType {
        modname: "hash",
        name: "hash_zero",
        f: hash_zero,
    };

    #[test_case]
    const TEST_hash_zero_is_empty: TestType = TestType {
        modname: "hash",
        name: "hash_zero_is_empty",
        f: hash_zero_is_empty,
    };

    #[test_case]
    const TEST_hash_one_is_empty: TestType = TestType {
        modname: "hash",
        name: "hash_one_is_empty",
        f: hash_one_is_empty,
    };

    #[test_case]
    const TEST_hash_zero_asset: TestType = TestType {
        modname: "hash",
        name: "hash_zero_asset",
        f: hash_zero_asset,
    };

    #[test_case]
    const TEST_hash_one_asset: TestType = TestType {
        modname: "hash",
        name: "hash_one_asset",
        f: hash_one_asset,
    };
}
