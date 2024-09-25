use serde::{Deserializer, Serializer};

// use crate::internals::serde::de::IndexedDeserializer;

pub trait FromRobTop<'r>: Sized {
    fn from_robtop<D: Deserializer<'r>>(deserializer: D) -> Result<Self, D::Error>;
}

pub trait ToRobTop {
    fn to_robtop<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

pub trait RobTop<'r>: FromRobTop<'r> + ToRobTop {}

impl<'r, T> RobTop<'r> for T where T: FromRobTop<'r> + ToRobTop {}

pub trait RobTopString<'r>: RobTop<'r> {
    const DELIMITER: &'static str;
    const SEQUENCE: bool = false;

    fn from_robtop_string(string: &'r str) -> Self;
    fn to_robtop_string(&self) -> String;
}
