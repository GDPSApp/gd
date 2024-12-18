use serde::{Deserializer, Serializer};

use crate::internals::serde::{
    de::{self, IndexedDeserializer},
    ser::{self, IndexedSerializer},
};

pub trait FromRobTop<'r>: Sized {
    fn from_robtop<D: Deserializer<'r>>(deserializer: D) -> Result<Self, D::Error>;
}

pub trait FromRobTopOwned: for<'r> FromRobTop<'r> {}

impl<T> FromRobTopOwned for T where T: for<'r> FromRobTop<'r> {}

pub trait ToRobTop {
    fn to_robtop<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

pub trait RobTopString<'r>: FromRobTop<'r> + ToRobTop {
    const DELIMITER: &'static str;
    const SEQUENCE: bool = false;

    fn from_robtop_string(string: &'r str) -> Result<Self, de::Error> {
        let mut indexed_deserializer = IndexedDeserializer::builder()
            .source(string)
            .delimiter(Self::DELIMITER)
            .sequence(Self::SEQUENCE)
            .build();

        Self::from_robtop(&mut indexed_deserializer)
    }

    fn to_robtop_string(&self) -> Result<String, ser::Error> {
        let mut string = String::new();

        let mut indexed_serializer = IndexedSerializer::builder()
            .writer(&mut string)
            .delimiter(Self::DELIMITER)
            .sequence(Self::SEQUENCE)
            .build();

        self.to_robtop(&mut indexed_serializer)?;

        Ok(string)
    }
}
