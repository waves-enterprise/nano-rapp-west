/// Transaction version
#[derive(PartialEq)]
pub enum Version {
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    Undefined,
}

impl Version {
    pub fn from_u8(value: u8) -> Version {
        match value {
            1u8 => Version::V1,
            2u8 => Version::V2,
            3u8 => Version::V3,
            4u8 => Version::V4,
            _ => Version::Undefined,
        }
    }
}
