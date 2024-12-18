use core::fmt;
use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{entities::artist::Artist, primitives::id::Id};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Builder)]
pub struct Reference {
    pub id: Id,
    pub custom: bool,
}

impl Reference {
    pub fn is_custom(&self) -> bool {
        self.custom
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Song<'s> {
    pub id: Id,
    pub name: Cow<'s, str>,
    pub artist: Artist<'s>,
}

impl fmt::Display for Song<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(formatter)
    }
}
