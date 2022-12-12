use smol_base_x::{Base, Base58Btc};

use crate::crypto::secure_hash;
use crate::crypto::sodium;

pub const ADDRESS_LENGTH: usize = 26;
pub const PUBLIC_KEY_LENGTH: usize = 32;

const ADDRESS_VERSION: u8 = 1;

/// An account possessing a address.
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn new(bytes: [u8; ADDRESS_LENGTH]) -> Address {
        Address(bytes)
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    pub fn to_base58(&self, buf: &mut [u8]) {
        Base58Btc::encode_mut(self.0, buf).unwrap();
    }
}

/// An account possessing a public key.
#[derive(Clone)]
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

    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    pub fn to_address(&mut self, chain_id: u8) -> Address {
        let mut buf = [0u8; ADDRESS_LENGTH];
        buf[0] = ADDRESS_VERSION;
        buf[1] = chain_id;

        let mut public_key_hash = [0u8; 32];
        secure_hash(&mut self.0, 32, &mut public_key_hash);

        buf[2..22].clone_from_slice(&public_key_hash[..20]);

        let mut checksum = [0u8; 32];
        secure_hash(&mut buf[..22], 22, &mut checksum);

        buf[22..].clone_from_slice(&checksum[..4]);

        Address::new(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use nanos_sdk::TestType;

    fn account_address() -> Result<(), ()> {
        let bytes = [0; ADDRESS_LENGTH];
        let address = Address::new(bytes);

        if bytes == *address.to_bytes() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn account_public_key() -> Result<(), ()> {
        let bytes = [0; PUBLIC_KEY_LENGTH];
        let pk = PublicKeyAccount::new(bytes);

        if bytes == *pk.to_bytes() {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_account_address: TestType = TestType {
        modname: "account",
        name: "account_address",
        f: account_address,
    };

    #[test_case]
    const TEST_account_public_key: TestType = TestType {
        modname: "account",
        name: "account_public_key",
        f: account_public_key,
    };
}
