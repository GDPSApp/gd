use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{cow::wrap_into_owned, types::Id};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Credentials<'c> {
    pub account_id: Id,
    pub id: Id,
    pub name: Cow<'c, str>,
    pub hashed_password: Cow<'c, str>,
}

impl Credentials<'_> {
    pub fn into_owned(self) -> Credentials<'static> {
        Credentials {
            account_id: self.account_id,
            id: self.id,
            name: wrap_into_owned(self.name),
            hashed_password: wrap_into_owned(self.hashed_password),
        }
    }
}
