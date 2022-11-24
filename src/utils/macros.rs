/// Creating a simple screen from a title and a message
#[macro_export]
macro_rules! single_screen {
    ($title: expr, $message: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        $cursor += 1;

        $titles[$cursor - 1..$cursor].clone_from_slice(&[&$title]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&$message]);
    };
}

#[macro_export]
macro_rules! address_screen {
    ($address: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        let a1 = unsafe { str::from_utf8_unchecked(&$address[0..16]) };
        let a2 = unsafe { str::from_utf8_unchecked(&$address[16..32]) };
        let a3 = unsafe { str::from_utf8_unchecked(&$address[32..]) };

        $cursor += 1;
        $titles[$cursor - 1..$cursor].clone_from_slice(&["Address (1/3)"]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&a1]);

        $cursor += 1;
        $titles[$cursor - 1..$cursor].clone_from_slice(&["Address (2/3)"]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&a2]);

        $cursor += 1;
        $titles[$cursor - 1..$cursor].clone_from_slice(&["Address (3/3)"]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&a3]);
    };
}

/// Creating a screen or screens for hashes
#[macro_export]
macro_rules! hash_screen {
    ($title: expr, $hash: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        match $hash {
            Some(_hash) => {
                // TODO: Display hash
                $cursor += 1;
                $titles[$cursor - 1..$cursor].clone_from_slice(&[&$title]);
                $messages[$cursor - 1..$cursor].clone_from_slice(&[&"None"]);
            }
            None => {
                $cursor += 1;
                $titles[$cursor - 1..$cursor].clone_from_slice(&[&$title]);
                $messages[$cursor - 1..$cursor].clone_from_slice(&[&"WEST"]);
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
