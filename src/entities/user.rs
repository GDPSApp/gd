use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::primitives::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Builder)]
pub struct Reference<'r> {
    pub id: Id,
    pub name: Cow<'r, str>,
    pub account_id: Id,
}

pub struct User;
