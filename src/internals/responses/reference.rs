use bon::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    internals::serde::robtop::{FromRobTop, RobTopString, ToRobTop},
    primitives::id::Id,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Reference {
    pub account_id: Id,
    pub id: Id,
}

impl<'r> FromRobTop<'r> for Reference {
    fn from_robtop<D: Deserializer<'r>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(deserializer)
    }
}

impl ToRobTop for Reference {
    fn to_robtop<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize(serializer)
    }
}

impl<'r> RobTopString<'r> for Reference {
    const DELIMITER: &'static str = ",";
    const SEQUENCE: bool = true;
}
