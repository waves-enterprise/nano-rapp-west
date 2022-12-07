use crate::transaction::data_entry::DataEntry;

pub struct Deserializer<'a> {
    buffer: &'a [u8],
}

impl<'a> Deserializer<'a> {
    pub fn new(bytes: &[u8]) -> Deserializer {
        Deserializer { buffer: bytes }
    }

    pub fn get_byte(self: &mut Deserializer<'a>, value: &mut u8) -> Deserializer {
        let (byte, buffer) = get_byte(self.buffer);
        *value = byte;
        Deserializer { buffer }
    }

    pub fn get_bool(self: &mut Deserializer<'a>, value: &mut bool) -> Deserializer {
        let (byte, buffer) = get_byte(self.buffer);
        *value = to_bool(byte);
        Deserializer { buffer }
    }

    pub fn get_bytes(self: &mut Deserializer<'a>, value: &mut [u8], size: usize) -> Deserializer {
        let buffer = get_bytes(self.buffer, value, size);
        Deserializer { buffer }
    }

    pub fn get_bytes_flag(
        self: &mut Deserializer<'a>,
        value: &mut [u8],
        size: usize,
    ) -> Deserializer {
        let (byte, buffer) = get_byte(self.buffer);
        let flag = to_bool(byte);

        if flag {
            let buffer = get_bytes(buffer, value, size);
            Deserializer { buffer }
        } else {
            Deserializer { buffer }
        }
    }

    pub fn get_string(self: &mut Deserializer<'a>, value: &mut [u8]) -> Deserializer {
        let buffer = get_string(self.buffer, value);
        Deserializer { buffer }
    }

    // TODO: may not be needed in the future
    pub fn skip_string(self: &mut Deserializer<'a>) -> Deserializer {
        let buffer = skip_string(self.buffer);
        Deserializer { buffer }
    }

    // TODO: change to get_params
    // TODO: may not be needed in the future
    pub fn skip_params(self: &mut Deserializer<'a>) -> Deserializer {
        let (byte, buffer) = get_byte(self.buffer);
        let flag = to_bool(byte);

        if flag {
            let (mut count, mut buffer) = get_u8(buffer);
            if count > 0 {
                while count > 0 {
                    // TODO: parse key and value
                    buffer = skip_string(buffer);
                    let (_, buf) = skip_value(buffer);
                    buffer = buf;
                    count -= 1;
                }
                Deserializer { buffer }
            } else {
                Deserializer { buffer }
            }
        } else {
            Deserializer { buffer }
        }
    }
}

fn get_byte(buffer: &[u8]) -> (u8, &[u8]) {
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

fn get_u8(buffer: &[u8]) -> (usize, &[u8]) {
    let mut temp = [0u8; 1];
    let buffer = get_bytes(buffer, &mut temp, 1);

    (u8::from_be_bytes(temp) as usize, buffer)
}

fn get_u16(buffer: &[u8]) -> (usize, &[u8]) {
    let mut temp = [0u8; 2];
    let buffer = get_bytes(buffer, &mut temp, 2);

    (u16::from_be_bytes(temp) as usize, buffer)
}

fn get_u64(buffer: &[u8]) -> (u64, &[u8]) {
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
fn skip_string(buffer: &[u8]) -> &[u8] {
    let (length, buffer) = get_u16(buffer);

    if length > 0 {
        &buffer[length..]
    } else {
        buffer
    }
}

// TODO: change to get_value
// TODO: may not be needed in the future
fn skip_value(buffer: &[u8]) -> (DataEntry, &[u8]) {
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
    byte == 1u8
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
