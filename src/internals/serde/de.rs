use std::{fmt, str::Split};

use bon::{bon, Builder};
use miette::Diagnostic;
use serde::{
    de::{DeserializeSeed, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct IndexedDeserializer<'de> {
    splitter: Split<'de, &'de str>,
    source: &'de str,
    delimiter: &'de str,
    sequence: bool,
    position: usize,
}

#[derive(Debug, Error, Diagnostic)]
#[error("unexpected EOF while parsing")]
#[diagnostic(code(gd::internals::serde::de::eof))]
pub struct UnexpectedEofError;

#[derive(Debug, Error, Diagnostic, Builder)]
#[diagnostic(
    code(gd::internals::serde::de::custom),
    help("see the report for more information")
)]
pub struct CustomError {
    pub message: String,
    pub index: Option<String>,
    pub value: Option<String>,
}

impl fmt::Display for CustomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = self.message();

        match (self.index(), self.value()) {
            (None, None) => write!(formatter, "{message}"),
            (Some(index), None) => write!(formatter, "{message} (at index `{index}`)"),
            (None, Some(value)) => write!(formatter, "{message} (on value `{value}`)"),
            (Some(index), Some(value)) => write!(
                formatter,
                "{message} (at index `{index}`, on value `{value}`)"
            ),
        }
    }
}

impl CustomError {
    pub fn new(message: String) -> Self {
        Self::builder().message(message).build()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn index(&self) -> Option<&str> {
        self.index.as_deref()
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl serde::de::Error for CustomError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::new(message.to_string())
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("unsupported deserialization function: {name}")]
#[diagnostic(
    code(gd::internals::serde::de::unsupported),
    help("make sure only supported types are deserialized")
)]
pub struct UnsupportedError {
    pub name: &'static str,
}

impl UnsupportedError {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    UnexpectedEof(#[from] UnexpectedEofError),
    Custom(#[from] CustomError),
    Unsupported(#[from] UnsupportedError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse")]
#[diagnostic(code(gd::internals::serde::de))]
pub struct Error {
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    pub fn unexpected_eof(error: UnexpectedEofError) -> Self {
        Self::new(error.into())
    }

    pub fn custom(error: CustomError) -> Self {
        Self::new(error.into())
    }

    pub fn unsupported(error: UnsupportedError) -> Self {
        Self::new(error.into())
    }

    pub fn new_unexpected_eof() -> Self {
        Self::unexpected_eof(UnexpectedEofError)
    }

    pub fn new_custom(message: String) -> Self {
        Self::custom(CustomError::new(message))
    }

    pub fn new_unsupported(name: &'static str) -> Self {
        Self::unsupported(UnsupportedError::new(name))
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::new_custom(message.to_string())
    }
}

#[bon]
impl<'de> IndexedDeserializer<'de> {
    #[builder]
    pub fn new(source: &'de str, delimiter: &'de str, sequence: bool) -> Self {
        let splitter = source.split(delimiter);
        let position = 0;

        Self {
            splitter,
            source,
            delimiter,
            sequence,
            position,
        }
    }
}

impl<'de> IndexedDeserializer<'de> {
    fn next(&mut self) -> Option<&'de str> {
        let string = self.splitter.next()?;

        self.position += string.len();

        Some(string)
    }

    fn is_next_empty(&self) -> bool {
        let delimiter = self.delimiter;

        let length = delimiter.len();

        let start = self.position + length;
        let end = start + length;

        &self.source[start..end] == delimiter
    }

    fn is_eof(&self) -> bool {
        self.source.len() <= self.position
    }
}

macro_rules! delegate_to_parse {
    ($lifetime: lifetime, $deserialize_method: ident, $visit_method: ident) => {
        fn $deserialize_method<V: Visitor<$lifetime>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let string = self.next().ok_or_else(Self::Error::new_unexpected_eof)?;

            let parsed = string
                .parse()
                .map_err(<Self::Error as serde::de::Error>::custom)?;

            visitor.$visit_method(parsed)
        }
    };
}

macro_rules! unsupported {
    ($name: ident) => {
        Err(Self::Error::new_unsupported(stringify!($name)))
    };
}

const FALSE: &str = "0";

impl<'i, 'de> Deserializer<'de> for &'i mut IndexedDeserializer<'de> {
    type Error = Error;

    delegate_to_parse!('de, deserialize_i8, visit_i8);
    delegate_to_parse!('de, deserialize_u8, visit_u8);
    delegate_to_parse!('de, deserialize_i16, visit_i16);
    delegate_to_parse!('de, deserialize_u16, visit_u16);
    delegate_to_parse!('de, deserialize_i32, visit_i32);
    delegate_to_parse!('de, deserialize_u32, visit_u32);
    delegate_to_parse!('de, deserialize_i64, visit_i64);
    delegate_to_parse!('de, deserialize_u64, visit_u64);

    delegate_to_parse!('de, deserialize_f32, visit_f32);
    delegate_to_parse!('de, deserialize_f64, visit_f64);

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_any)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let string = self.next().ok_or_else(Self::Error::new_unexpected_eof)?;

        visitor.visit_bool(!string.is_empty() && string != FALSE)
    }

    fn deserialize_char<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_char)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let string = self.next().ok_or_else(Self::Error::new_unexpected_eof)?;

        visitor.visit_borrowed_str(string)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_bytes)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_byte_buf)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.is_eof() || self.is_next_empty() {
            let _ = self.next();

            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_unit)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_unit_struct)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_newtype_struct)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _length: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_tuple)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _length: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_tuple_struct)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if self.sequence {
            self.deserialize_seq(visitor)
        } else {
            self.deserialize_map(visitor)
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unsupported!(deserialize_enum)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let _ = self.next();

        visitor.visit_none()
    }
}

fn eof_is_ok<T>(error: Error) -> Result<Option<T>, Error> {
    match error.source {
        ErrorSource::UnexpectedEof(_) => Ok(None),
        _ => Err(error),
    }
}

impl<'i, 'de> SeqAccess<'de> for &'i mut IndexedDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        seed.deserialize(&mut **self).map(Some).or_else(eof_is_ok)
    }
}

impl<'i, 'de> MapAccess<'de> for &'i mut IndexedDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        seed.deserialize(&mut **self).map(Some).or_else(eof_is_ok)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        seed.deserialize(&mut **self)
    }
}
