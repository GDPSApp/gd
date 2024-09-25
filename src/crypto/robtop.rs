use miette::Diagnostic;
use thiserror::Error;

use crate::crypto::{base64, utf8, xor};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Decode(#[from] base64::DecodeError),
    Utf8(#[from] utf8::Error),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to decode robtop into string")]
#[diagnostic(
    code(gd::crypto::robtop::decode_string),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    pub fn decode(error: base64::DecodeError) -> Self {
        Self::new(error.into())
    }

    pub fn utf8(error: utf8::Error) -> Self {
        Self::new(error.into())
    }
}

pub fn encode_mut<D: AsMut<[u8]>, K: AsRef<[u8]>>(mut data: D, key: K) -> String {
    fn encode_mut_inner(data: &mut [u8], key: &[u8]) -> String {
        xor::cyclic(&mut *data, key);

        base64::encode(data)
    }

    encode_mut_inner(data.as_mut(), key.as_ref())
}

pub fn encode<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> String {
    encode_mut(data.as_ref().to_owned(), key)
}

pub fn decode<D: AsRef<[u8]>, K: AsRef<[u8]>>(
    data: D,
    key: K,
) -> Result<Vec<u8>, base64::DecodeError> {
    fn decode_inner(data: &[u8], key: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
        let mut owned = base64::decode(data)?;

        xor::cyclic(&mut owned, key);

        Ok(owned)
    }

    decode_inner(data.as_ref(), key.as_ref())
}

pub fn decode_string<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> Result<String, Error> {
    fn decode_string_inner(data: &[u8], key: &[u8]) -> Result<String, Error> {
        let mut owned = base64::decode(data).map_err(Error::decode)?;

        xor::cyclic(&mut owned, key);

        utf8::convert(owned).map_err(Error::utf8)
    }

    decode_string_inner(data.as_ref(), key.as_ref())
}
