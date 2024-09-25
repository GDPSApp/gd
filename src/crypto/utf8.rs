use std::string::FromUtf8Error;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("invalid utf-8 was found")]
#[diagnostic(
    code(gd::crypto::base64::utf8),
    help("make sure the output is valid utf-8")
)]
pub struct Error(#[from] pub FromUtf8Error);

pub fn convert(vec: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(vec).map_err(Error)
}
