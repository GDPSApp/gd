use std::{borrow::Cow, convert::Infallible};

use base64::engine::{general_purpose::URL_SAFE, Engine};
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{crypto::utf8, thunk};

#[derive(Debug, Error, Diagnostic)]
#[error("failed to decode base64")]
#[diagnostic(code(gd::crypto::base64::decode), help("make sure input is valid"))]
pub struct DecodeError(#[from] pub base64::DecodeError);

pub fn encode<D: AsRef<[u8]>>(data: D) -> String {
    fn encode_inner(data: &[u8]) -> String {
        URL_SAFE.encode(data)
    }

    encode_inner(data.as_ref())
}

pub fn decode<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, DecodeError> {
    fn decode_inner(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
        URL_SAFE.decode(data).map_err(DecodeError)
    }

    decode_inner(data.as_ref())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Processor;

impl thunk::Processor for Processor {
    type ProcessError = DecodeError;
    type UnprocessError = Infallible;

    type Output<'a> = Cow<'a, [u8]>;

    fn process(unprocessed: Cow<'_, str>) -> Result<Self::Output<'_>, Self::ProcessError> {
        decode(unprocessed.as_ref()).map(Cow::Owned)
    }

    fn unprocess<'p>(
        processed: &'p Self::Output<'_>,
    ) -> Result<Cow<'p, str>, Self::UnprocessError> {
        Ok(Cow::Owned(encode(processed.as_ref())))
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Decode(#[from] DecodeError),
    Utf8(#[from] utf8::Error),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to decode base64 into string")]
#[diagnostic(
    code(gd::crypto::base64::decode_string),
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

    pub fn decode(error: DecodeError) -> Self {
        Self::new(error.into())
    }

    pub fn utf8(error: utf8::Error) -> Self {
        Self::new(error.into())
    }
}

pub fn decode_string<D: AsRef<[u8]>>(data: D) -> Result<String, Error> {
    let decoded = decode(data).map_err(Error::decode)?;

    utf8::convert(decoded).map_err(Error::utf8)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StringProcessor;

impl thunk::Processor for StringProcessor {
    type ProcessError = Error;
    type UnprocessError = Infallible;

    type Output<'a> = Cow<'a, str>;

    fn process(unprocessed: Cow<'_, str>) -> Result<Self::Output<'_>, Self::ProcessError> {
        decode_string(unprocessed.as_ref()).map(Cow::Owned)
    }

    fn unprocess<'p>(
        processed: &'p Self::Output<'_>,
    ) -> Result<Cow<'p, str>, Self::UnprocessError> {
        Ok(Cow::Owned(encode(processed.as_ref())))
    }
}
