/// Creating a simple screen from a title and a message
#[macro_export]
macro_rules! single_screen {
    ($title: expr, $message: expr, $cursor: ident, $titles: ident, $messages: ident) => {
        $cursor += 1;

        $titles[$cursor - 1..$cursor].clone_from_slice(&[&$title]);
        $messages[$cursor - 1..$cursor].clone_from_slice(&[&$message]);
    };
}

/// Creating a screen or screens for hashes
#[macro_export]
macro_rules! hash_screen {
    ($title: expr, $hash:expr, $cursor: ident, $titles: ident, $messages: ident) => {
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

/// Convert all the numbers from the transaction into strings
/// needed to display them on the screen
#[macro_export]
macro_rules! convert_numbers {
    ([$($value:expr),+], [$($variable:ident),+], $buf: ident) => {
        let mut sizes = [0; 4];
        let mut offset = 0;
        let mut position = 0;

        $(
            #[allow(unused_assignments)]
            {
                let value_size = utils::add_number_to_buf($value, offset, $buf);
                offset += value_size;
                sizes[position] = value_size;
                position += 1;
            }
        )+

        offset = 0;
        position = 0;

        $(
            #[allow(unused_assignments)]
            {
                $variable = unsafe { str::from_utf8_unchecked(&$buf[offset..offset + sizes[position]]) };
                offset += sizes[position];
                position += 1;
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_simple_test {
    ($tx:ident, $type_id:expr, $version:expr, $fee:expr) => {
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
