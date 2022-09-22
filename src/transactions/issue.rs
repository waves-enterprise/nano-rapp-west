use crate::single_screen;
use crate::transactions::*;

pub struct Issue {
    type_id: Type,
    version: Version,
}

impl<'a> Transaction<'a> for Issue {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Buffer::new(bytes);

        let mut type_id = 0_u8;
        let mut version = 0_u8;

        buffer.get_byte(&mut type_id).get_byte(&mut version);

        Issue {
            type_id: Type::from_u8(type_id),
            version: Version::from_u8(version),
        }
    }

    fn to_messages(&self, _buf: &'a mut [u8]) -> ([&'a str; MAX_SIZE], [&'a str; MAX_SIZE], usize) {
        let mut titles = [""; MAX_SIZE];
        let mut messages = [""; MAX_SIZE];

        let mut cursor: usize = 0;

        single_screen!("Review", "issue", cursor, titles, messages);

        (titles, messages, cursor)
    }
}
