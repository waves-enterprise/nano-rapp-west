/// Creating a simple screen from a title and a message
#[macro_export]
macro_rules! single_screen {
    ($title: expr, $message: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        $cursor += 1;

        $titles[$cursor - 1..$cursor].clone_from_slice(&[&$title]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&$message]);
    };
}

/// Creating three screens for a long message
#[macro_export]
macro_rules! three_screens {
    ($title: expr, $message: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        let title_bytes = $title.as_bytes();
        let title_len = title_bytes.len();

        let mut title_screen_1 = [0u8; 16];
        title_screen_1[..title_len].clone_from_slice(title_bytes);
        title_screen_1[title_len..title_len + 6].clone_from_slice(b" (1/3)");

        let mut title_screen_2 = [0u8; 16];
        title_screen_2[..title_len].clone_from_slice(title_bytes);
        title_screen_2[title_len..title_len + 6].clone_from_slice(b" (2/3)");

        let mut title_screen_3 = [0u8; 16];
        title_screen_3[..title_len].clone_from_slice(title_bytes);
        title_screen_3[title_len..title_len + 6].clone_from_slice(b" (3/3)");

        let title_1 = unsafe { str::from_utf8_unchecked(&title_screen_1) };
        let title_2 = unsafe { str::from_utf8_unchecked(&title_screen_2) };
        let title_3 = unsafe { str::from_utf8_unchecked(&title_screen_3) };

        let m1 = unsafe { str::from_utf8_unchecked(&$message[0..16]) };
        let m2 = unsafe { str::from_utf8_unchecked(&$message[16..32]) };
        let m3 = unsafe { str::from_utf8_unchecked(&$message[32..]) };

        single_screen!(title_1, m1, $cursor, $titles, $messages);
        single_screen!(title_2, m2, $cursor, $titles, $messages);
        single_screen!(title_3, m3, $cursor, $titles, $messages);
    };
}

/// Creating a screen or screens for asset
#[macro_export]
macro_rules! asset_screen {
    ($title: expr, $hash: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        let mut bytes = [0u8; 44];

        match $hash {
            Some(hash) => {
                hash.to_base58(&mut bytes);
            }
            None => (),
        }

        let title_bytes = $title.as_bytes();
        let title_len = title_bytes.len();

        let mut title_screen_1 = [0u8; 16];
        title_screen_1[..title_len].clone_from_slice(title_bytes);
        title_screen_1[title_len..title_len + 6].clone_from_slice(b" (1/3)");

        let mut title_screen_2 = [0u8; 16];
        title_screen_2[..title_len].clone_from_slice(title_bytes);
        title_screen_2[title_len..title_len + 6].clone_from_slice(b" (2/3)");

        let mut title_screen_3 = [0u8; 16];
        title_screen_3[..title_len].clone_from_slice(title_bytes);
        title_screen_3[title_len..title_len + 6].clone_from_slice(b" (3/3)");

        let title_1 = unsafe { str::from_utf8_unchecked(&title_screen_1) };
        let title_2 = unsafe { str::from_utf8_unchecked(&title_screen_2) };
        let title_3 = unsafe { str::from_utf8_unchecked(&title_screen_3) };

        let m1 = unsafe { str::from_utf8_unchecked(&bytes[0..16]) };
        let m2 = unsafe { str::from_utf8_unchecked(&bytes[16..32]) };
        let m3 = unsafe { str::from_utf8_unchecked(&bytes[32..]) };

        match $hash {
            Some(_) => {
                single_screen!(title_1, m1, $cursor, $titles, $messages);
                single_screen!(title_2, m2, $cursor, $titles, $messages);
                single_screen!(title_3, m3, $cursor, $titles, $messages);
            }
            None => {
                single_screen!($title, "WEST", $cursor, $titles, $messages);
            }
        }
    };
}

/// Converts a number from a transaction into the string
/// needed to display it on the screen
#[macro_export]
macro_rules! convert_number_to_str {
    ($value: expr, $variable: ident, $buf: ident) => {
        let (buf, buf_size) = number_to_formatted_bytes($value, &mut $buf);
        $variable = unsafe { str::from_utf8_unchecked(&buf[..buf_size]) };
    };
}

/// Creating additional methods for the transaction and test functions
#[macro_export]
macro_rules! impl_transactions_test {
    ($tx: ident, $type_id: expr, $version: expr, $fee: expr) => {
        #[cfg(test)]
        impl $tx {
            pub fn get_type_id(&self) -> Type {
                self.type_id
            }

            pub fn get_version(&self) -> Version {
                self.version
            }

            pub fn get_fee(&self) -> u64 {
                self.fee
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            use nanos_sdk::TestType;

            fn run() -> Result<(), ()> {
                let tx = $tx::from_bytes(&BYTES);

                let mut result = false;

                result = tx.get_type_id() == $type_id;

                result = tx.get_version() == $version;

                result = tx.get_fee() == $fee;

                if result {
                    Ok(())
                } else {
                    Err(())
                }
            }

            #[test_case]
            const TEST: TestType = TestType {
                modname: "transactions",
                name: stringify!($tx),
                f: run,
            };
        }
    };
}
