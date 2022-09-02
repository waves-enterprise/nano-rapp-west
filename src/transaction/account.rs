const ADDRESS_VERSION: u8 = 1;
const ADDRESS_LENGTH: usize = 26;

pub const PUBLIC_KEY_LENGTH: usize = 32;

/// MAINNET chainID
pub const MAINNET: u8 = b'W';
/// TESTNET chainID
pub const TESTNET: u8 = b'T';
/// STAGENET chainID
pub const STAGENET: u8 = b'S';

/// An account possessing a address.
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn new(bytes: [u8; ADDRESS_LENGTH]) -> Address {
        Address(bytes)
    }

    pub fn chain_id(&self) -> u8 {
        self.0[1]
    }

    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }
}

/// An account possessing a public key.
pub struct PublicKeyAccount([u8; PUBLIC_KEY_LENGTH]);

impl PublicKeyAccount {
    pub fn new(bytes: [u8; PUBLIC_KEY_LENGTH]) -> PublicKeyAccount {
        PublicKeyAccount(bytes)
    }

    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }
}
