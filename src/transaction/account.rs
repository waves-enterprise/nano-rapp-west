use crate::sodium;

pub const ADDRESS_LENGTH: usize = 26;
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// An account possessing a address.
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn new(bytes: [u8; ADDRESS_LENGTH]) -> Address {
        Address(bytes)
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

    // Convert ed25519 public key to curve25519
    pub fn from_ed25519(public_key: &[u8]) -> PublicKeyAccount {
        let public_key_be = Self::from_public_key_le(public_key);

        let result = sodium::ed25519_pk_to_curve25519(public_key_be);

        PublicKeyAccount(result)
    }

    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    // Converts little endian 65 byte (0x4 32X 32Y)
    // public key to 32 byte Y big endian form
    // (for other applications)
    fn from_public_key_le(public_key: &[u8]) -> [u8; 32] {
        let mut public_key_be = [0u8; 32];

        for i in 0..32 {
            public_key_be[i] = public_key[64 - i];
        }

        if public_key[32] & 1 != 0 {
            public_key_be[31] |= 0x80;
        }

        public_key_be
    }
}
