use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    cow::wrap_into_owned, internals::responses::reference::Reference as InternalReference,
    primitives::id::Id,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Credentials<'c> {
    pub name: Cow<'c, str>,
    pub hashed_password: Cow<'c, str>,
}

impl Credentials<'_> {
    pub fn into_owned(self) -> Credentials<'static> {
        Credentials {
            name: wrap_into_owned(self.name),
            hashed_password: wrap_into_owned(self.hashed_password),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Reference {
    pub account_id: Id,
    pub id: Id,
}

impl From<InternalReference> for Reference {
    fn from(internal: InternalReference) -> Self {
        Self {
            account_id: internal.account_id,
            id: internal.id,
        }
    }
}

impl From<Reference> for InternalReference {
    fn from(this: Reference) -> Self {
        Self {
            account_id: this.account_id,
            id: this.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Auth<'a> {
    pub reference: Reference,
    pub credentials: Credentials<'a>,
}

impl Auth<'_> {
    pub fn into_owned(self) -> Auth<'static> {
        Auth {
            reference: self.reference,
            credentials: self.credentials.into_owned(),
        }
    }
}
