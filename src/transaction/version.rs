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

#[cfg(test)]
mod tests {
    use super::*;

    use nanos_sdk::TestType;

    fn version_1() -> Result<(), ()> {
        let v1 = 1u8;
        let version = Version::from_u8(v1);

        if Version::V1 == version {
            Ok(())
        } else {
            Err(())
        }
    }

    fn version_42() -> Result<(), ()> {
        let v42 = 42u8;
        let version = Version::from_u8(v42);

        if Version::Undefined == version {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test_case]
    const TEST_version_1: TestType = TestType {
        modname: "version",
        name: "version_1",
        f: version_1,
    };

    #[test_case]
    const TEST_version_42: TestType = TestType {
        modname: "version",
        name: "version_42",
        f: version_42,
    };
}
