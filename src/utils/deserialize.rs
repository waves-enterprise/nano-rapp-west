use crate::transaction::data_entry::DataEntry;

pub struct Buffer<'a> {
    buffer: &'a [u8],
}

impl<'a> Buffer<'a> {
    pub fn new(bytes: &[u8]) -> Buffer {
        Buffer { buffer: bytes }
    }

    pub fn get_byte(self: &mut Buffer<'a>, value: &mut u8) -> Buffer {
        let (byte, buffer) = get_byte(self.buffer);
        *value = byte;
        Buffer { buffer }
    }

    pub fn get_bool(self: &mut Buffer<'a>, value: &mut bool) -> Buffer {
        let (byte, buffer) = get_byte(self.buffer);
        *value = to_bool(byte);
        Buffer { buffer }
    }

    pub fn get_bytes(self: &mut Buffer<'a>, value: &mut [u8], size: usize) -> Buffer {
        let buffer = get_bytes(self.buffer, value, size);
        Buffer { buffer }
    }

    pub fn get_bytes_flag(self: &mut Buffer<'a>, value: &mut [u8], size: usize) -> Buffer {
        let (byte, buffer) = get_byte(self.buffer);
        let flag = to_bool(byte);

        if flag {
            let buffer = get_bytes(buffer, value, size);
            Buffer { buffer }
        } else {
            Buffer { buffer }
        }
    }

    pub fn get_string(self: &mut Buffer<'a>, value: &mut [u8]) -> Buffer {
        let buffer = get_string(self.buffer, value);
        Buffer { buffer }
    }

    // TODO: may not be needed in the future
    pub fn skip_string(self: &mut Buffer<'a>) -> Buffer {
        let buffer = skip_string(self.buffer);
        Buffer { buffer }
    }

    // TODO: change to get_params
    // TODO: may not be needed in the future
    pub fn skip_params(self: &mut Buffer<'a>) -> Buffer {
        let (byte, buffer) = get_byte(self.buffer);
        let flag = to_bool(byte);

        if flag {
            let (mut count, mut buffer) = get_u8(buffer);
            if count > 0 {
                while count > 0 {
                    // TODO: parse key and value
                    buffer = skip_string(buffer);
                    let (value, buf) = skip_value(buffer);
                    buffer = buf;
                    count -= 1;
                }
                Buffer { buffer }
            } else {
                Buffer { buffer }
            }
        } else {
            Buffer { buffer }
        }
    }
}

fn get_byte<'a>(buffer: &'a [u8]) -> (u8, &'a [u8]) {
    match buffer.first() {
        Some(byte) => (*byte, &buffer[1..]),
        None => (0u8, buffer),
    }
}

fn get_bytes<'a>(buffer: &'a [u8], value: &mut [u8], size: usize) -> &'a [u8] {
    match buffer.get(..size) {
        Some(bytes) => {
            value[..size].clone_from_slice(&bytes[..size]);
            &buffer[size..]
        }
        None => buffer,
    }
}

fn get_u8<'a>(buffer: &'a [u8]) -> (usize, &'a [u8]) {
    let mut temp = [0u8; 1];
    let buffer = get_bytes(buffer, &mut temp, 1);

    (u8::from_be_bytes(temp) as usize, buffer)
}

fn get_u16<'a>(buffer: &'a [u8]) -> (usize, &'a [u8]) {
    let mut temp = [0u8; 2];
    let buffer = get_bytes(buffer, &mut temp, 2);

    (u16::from_be_bytes(temp) as usize, buffer)
}

fn get_u64<'a>(buffer: &'a [u8]) -> (u64, &'a [u8]) {
    let mut temp = [0u8; 8];
    let buffer = get_bytes(buffer, &mut temp, 8);

    (u64::from_be_bytes(temp), buffer)
}

fn get_string<'a>(buffer: &'a [u8], value: &mut [u8]) -> &'a [u8] {
    let (length, buffer) = get_u16(buffer);

    if length > 0 {
        get_bytes(buffer, value, length)
    } else {
        buffer
    }
}

// TODO: may not be needed in the future
fn skip_string<'a>(buffer: &'a [u8]) -> &'a [u8] {
    let (length, buffer) = get_u16(buffer);

    if length > 0 {
        &buffer[length..]
    } else {
        buffer
    }
}

// TODO: change to get_value
// TODO: may not be needed in the future
fn skip_value<'a>(buffer: &'a [u8]) -> (DataEntry, &'a [u8]) {
    let (byte, buffer) = get_byte(buffer);

    if byte == 0u8 {
        let (value, buffer) = get_u64(buffer);
        (DataEntry::Integer(value), buffer)
    } else if byte == 1u8 {
        let (byte, buffer) = get_byte(buffer);
        (DataEntry::Boolean(to_bool(byte)), buffer)
    } else if byte == 2u8 {
        (DataEntry::Binary, skip_string(buffer))
    } else if byte == 3u8 {
        (DataEntry::String, skip_string(buffer))
    } else {
        (DataEntry::Undefined, buffer)
    }
}

fn to_bool(byte: u8) -> bool {
    if byte == 1u8 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use nanos_sdk::TestType;

    const BYTES: [u8; 12] = [1u8, 1u8, 2u8, 2u8, 1u8, 2u8, 2u8, 0u8, 3u8, 2u8, 2u8, 2u8];
    const SKIP_STRING_BYTES: [u8; 7] = [0u8, 4u8, 1u8, 1u8, 1u8, 1u8, 42u8];
    const SKIP_PARAMS_BYTES: [u8; 8] = [1u8, 1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 42u8];

    fn basic() -> Result<(), ()> {
        let mut buffer = Buffer::new(&BYTES);

        let mut byte = 0u8;
        let mut bool_value = false;
        let mut bytes = [0u8; 2];
        let mut bytes_flag = [0u8; 2];
        let mut string = [0u8; 3];

        buffer
            .get_byte(&mut byte)
            .get_bool(&mut bool_value)
            .get_bytes(&mut bytes, 2)
            .get_bytes_flag(&mut bytes_flag, 2)
            .get_string(&mut string);

        let mut result = false;
        result = 1 == byte;
        result = true == bool_value;
        result = [2u8; 2] == bytes;
        result = [2u8; 2] == bytes_flag;
        result = [2u8; 3] == string;

        if result {
            Ok(())
        } else {
            Err(())
        }
    }

    fn skip_string() -> Result<(), ()> {
        let mut buffer = Buffer::new(&SKIP_STRING_BYTES);

        let mut byte = 0u8;

        buffer.skip_string().get_byte(&mut byte);

        if 42 == byte {
            Ok(())
        } else {
            Err(())
        }
    }

    fn skip_params() -> Result<(), ()> {
        let mut buffer = Buffer::new(&SKIP_PARAMS_BYTES);

        let mut byte = 0u8;

        buffer.skip_params().get_byte(&mut byte);

        if 42 == byte {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_basic: TestType = TestType {
        modname: "deserialize",
        name: "basic",
        f: basic,
    };

    #[test_case]
    const TEST_skip_string: TestType = TestType {
        modname: "deserialize",
        name: "skip_string",
        f: skip_string,
    };

    #[test_case]
    const TEST_skip_params: TestType = TestType {
        modname: "deserialize",
        name: "skip_params",
        f: skip_params,
    };
}
