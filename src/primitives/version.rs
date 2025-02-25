use serde::{Deserialize, Serialize};

pub const BASE: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(from = "u8", into = "u8")]
pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    pub const fn new(major: u8, minor: u8) -> Option<Self> {
        if is_convertible(major, minor) {
            Some(Self { major, minor })
        } else {
            None
        }
    }

    pub const fn from_value(value: u8) -> Self {
        let (major, minor) = (value / BASE, value % BASE);

        Self::new(major, minor)
    }

    pub const fn into_value(self) -> u8 {
        self.major * BASE + self.minor
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        Self::from_value(value)
    }
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        version.into_value()
    }
}

pub const MAX_VALUE: u8 = u8::MAX;

pub const MAX_MINOR: u8 = BASE - 1;
pub const MAX_MAJOR: u8 = MAX_VALUE / BASE;
pub const MAX_MINOR_IF_MAX_MAJOR: u8 = MAX_VALUE % BASE;

pub const fn is_convertible(major: u8, minor: u8) -> bool {
    true
}
