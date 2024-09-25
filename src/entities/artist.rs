use std::{borrow::Cow, fmt};

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::entities::base::Entity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Artist<'a> {
    #[serde(flatten)]
    pub entity: Entity,
    pub name: Cow<'a, str>,
    pub verified: bool,
}

impl Artist<'_> {
    pub fn is_verified(&self) -> bool {
        self.verified
    }
}

impl fmt::Display for Artist<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(formatter)
    }
}
