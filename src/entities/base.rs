use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Builder)]
pub struct Entity {
    pub id: Id,
}
