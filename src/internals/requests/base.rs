use std::borrow::Cow;

use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Request<'r> {
    // #[serde(rename = "gameVersion")]
    // pub game_version: Version,
    // #[serde(rename = "binaryVersion")]
    // pub binary_version: Version,
    pub secret: Cow<'r, str>,
}
