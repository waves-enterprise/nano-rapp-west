/// Transaction version
#[derive(Clone, Copy, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use nanos_sdk::TestType;

    fn version() -> Result<(), ()> {
        if Version::V1 == Version::from_u8(1u8) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn version_undefined() -> Result<(), ()> {
        if Version::Undefined == Version::from_u8(42u8) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_version: TestType = TestType {
        modname: "version",
        name: "version",
        f: version,
    };

    #[test_case]
    const TEST_version_undefined: TestType = TestType {
        modname: "version",
        name: "version_undefined",
        f: version_undefined,
    };
}
