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
            $variable = unsafe { str::from_utf8_unchecked(&$buf[offset..offset + sizes[position]]) };
            offset += sizes[position];
            position += 1;
        )+
    };
}
