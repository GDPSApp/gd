use core::fmt;
use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::entities::{artist::Artist, base::Entity};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Builder)]
pub struct SongReference {
    #[serde(flatten)]
    pub entity: Entity,
    pub custom: bool,
}

impl SongReference {
    pub fn is_custom(&self) -> bool {
        self.custom
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Song<'s> {
    #[serde(flatten)]
    pub entity: Entity,
    pub name: Cow<'s, str>,
    pub artist: Artist<'s>,
}

impl fmt::Display for Song<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(formatter)
    }
}
