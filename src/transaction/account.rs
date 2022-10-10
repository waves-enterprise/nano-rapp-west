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

    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
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
