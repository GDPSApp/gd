use std::fmt;

use bon::bon;
use dtoa::{Buffer as FloatBuffer, Float};
use itoa::{Buffer as IntegerBuffer, Integer};
use miette::Diagnostic;
use serde::{
    ser::{Impossible, SerializeStruct},
    Serialize, Serializer,
};
use thiserror::Error;

use crate::crypto::base64;

#[derive(Debug, Clone)]
pub struct IndexedSerializer<W> {
    writer: W,
    delimiter: &'static str,
    sequence: bool,
    start: bool,
}

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
#[diagnostic(
    code(gd::internals::serde::ser::custom),
    help("see the report for more information")
)]
pub struct CustomError {
    pub message: String,
}

impl CustomError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl serde::ser::Error for CustomError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::new(message.to_string())
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("unsupported serialization function: {name}")]
#[diagnostic(
    code(gd::internals::serde::ser::unsupported),
    help("make sure only supported types are serialized")
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
#[error("failed to append")]
#[diagnostic(
    code(gd::internals::serde::ser::append),
    help("make sure everything is correct")
)]
pub struct AppendError;

impl From<fmt::Error> for AppendError {
    fn from(_fmt: fmt::Error) -> Self {
        Self
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Custom(#[from] CustomError),
    Unsupported(#[from] UnsupportedError),
    Append(#[from] AppendError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("error encountered during serialization")]
#[diagnostic(
    code(gd::internals::serde::ser),
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

    pub fn custom(error: CustomError) -> Self {
        Self::new(error.into())
    }

    pub fn unsupported(error: UnsupportedError) -> Self {
        Self::new(error.into())
    }

    pub fn append(error: AppendError) -> Self {
        Self::new(error.into())
    }

    pub fn new_custom(message: String) -> Self {
        Self::custom(CustomError::new(message))
    }

    pub fn new_unsupported(name: &'static str) -> Self {
        Self::unsupported(UnsupportedError::new(name))
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::new_custom(message.to_string())
    }
}

#[bon]
impl<W> IndexedSerializer<W> {
    #[builder]
    pub fn new(writer: W, delimiter: &'static str, sequence: bool) -> Self {
        let start = true;

        Self {
            writer,
            delimiter,
            sequence,
            start,
        }
    }
}

impl<W: fmt::Write> IndexedSerializer<W> {
    fn start_or_append_delimiter(&mut self) -> Result<(), AppendError> {
        if self.start {
            self.start = false;
        } else {
            self.writer.write_str(self.delimiter)?;
        };

        Ok(())
    }

    fn append(&mut self, string: &str) -> Result<(), AppendError> {
        self.start_or_append_delimiter()?;

        self.writer.write_str(string)?;

        Ok(())
    }

    fn append_char(&mut self, character: char) -> Result<(), AppendError> {
        self.start_or_append_delimiter()?;

        self.writer.write_char(character)?;

        Ok(())
    }

    fn append_integer<I: Integer>(&mut self, integer: I) -> Result<(), AppendError> {
        self.append(IntegerBuffer::new().format(integer))
    }

    fn append_float<F: Float>(&mut self, float: F) -> Result<(), AppendError> {
        self.append(FloatBuffer::new().format(float))
    }
}

pub const FALSE: &str = "0";
pub const TRUE: &str = "1";

macro_rules! serialize_via_append_integer {
    ($name: ident, $type: ty) => {
        fn $name(self, value: $type) -> Result<Self::Ok, Self::Error> {
            self.append_integer(value).map_err(Self::Error::append)
        }
    };
}

macro_rules! serialize_via_append_float {
    ($name: ident, $type: ty) => {
        fn $name(self, value: $type) -> Result<Self::Ok, Self::Error> {
            self.append_float(value).map_err(Self::Error::append)
        }
    };
}

macro_rules! unsupported {
    ($name: ident) => {
        Err(Self::Error::new_unsupported(stringify!($name)))
    };
}

impl<'i, W: fmt::Write> Serializer for &'i mut IndexedSerializer<W> {
    type Error = Error;

    type Ok = ();

    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(if value { TRUE } else { FALSE })
    }

    serialize_via_append_integer!(serialize_i8, i8);
    serialize_via_append_integer!(serialize_u8, u8);
    serialize_via_append_integer!(serialize_i16, i16);
    serialize_via_append_integer!(serialize_u16, u16);
    serialize_via_append_integer!(serialize_i32, i32);
    serialize_via_append_integer!(serialize_u32, u32);
    serialize_via_append_integer!(serialize_i64, i64);
    serialize_via_append_integer!(serialize_u64, u64);

    serialize_via_append_float!(serialize_f32, f32);
    serialize_via_append_float!(serialize_f64, f64);

    fn serialize_char(self, character: char) -> Result<Self::Ok, Self::Error> {
        self.append_char(character).map_err(Self::Error::append)
    }

    fn serialize_str(self, string: &str) -> Result<Self::Ok, Self::Error> {
        self.append(string).map_err(Self::Error::append)
    }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        let string = base64::encode(bytes);

        self.serialize_str(&string)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.start_or_append_delimiter()
            .map_err(Self::Error::append)
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unsupported!(serialize_unit)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unsupported!(serialize_unit_struct)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unsupported!(serialize_unit_variant)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsupported!(serialize_newtype_struct)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unsupported!(serialize_newtype_variant)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unsupported!(serialize_seq)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unsupported!(serialize_tuple)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unsupported!(serialize_tuple_struct)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unsupported!(serialize_tuple_variant)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unsupported!(serialize_map)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unsupported!(serialize_struct_variant)
    }
}

impl<'s, W: fmt::Write> SerializeStruct for &'s mut IndexedSerializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if self.sequence {
            value.serialize(&mut **self)
        } else {
            self.serialize_str(key)
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
