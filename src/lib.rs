#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod debug;
mod error;
mod map;
mod seq;
mod structure;
mod tuple;

use alloc::format;
use alloc::string::String;
use core::fmt::{self, Debug, Formatter};
use serde::ser::{self, Serialize, SerializeTupleStruct};

pub use debug::debug;
pub use error::Error;
pub use serdebug_derive::SerDebug;

/// A [`Serializer`](::serde::Serializer)-compatible wrapper for a [`Formatter`].
pub struct Serializer<'a, 'b: 'a>(pub &'a mut Formatter<'b>);

macro_rules! simple_impl {
    ($(fn $name:ident ( $v:ident : $ty:ty );)*) => {
        $(fn $name(self, $v: $ty) -> Result<Self::Ok, Self::Error> {
            Ok($v.fmt(self.0)?)
        })*
    };
}

impl<'a, 'b: 'a> ser::Serializer for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = seq::Serializer<'a, 'b>;
    type SerializeTuple = tuple::Serializer<'a, 'b>;
    type SerializeTupleStruct = tuple::Serializer<'a, 'b>;
    type SerializeTupleVariant = tuple::Serializer<'a, 'b>;
    type SerializeMap = map::Serializer<'a, 'b>;
    type SerializeStruct = structure::Serializer<'a, 'b>;
    type SerializeStructVariant = structure::Serializer<'a, 'b>;

    simple_impl! {
        fn serialize_bool(v: bool);
        fn serialize_i8(v: i8);
        fn serialize_i16(v: i16);
        fn serialize_i32(v: i32);
        fn serialize_i64(v: i64);
        fn serialize_u8(v: u8);
        fn serialize_u16(v: u16);
        fn serialize_u32(v: u32);
        fn serialize_u64(v: u64);
        fn serialize_f32(v: f32);
        fn serialize_f64(v: f64);
        fn serialize_char(v: char);
        fn serialize_str(v: &str);
        fn serialize_bytes(v: &[u8]);
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct("None")
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_struct("Some", value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().fmt(self.0)?)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(name, 0)?.end()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let mut tuple = self.serialize_tuple_struct(name, 1)?;
        tuple.serialize_field(value)?;
        tuple.end()
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_struct(variant, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(seq::Serializer::new(self.0))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(tuple::Serializer::new(self.0, name))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_tuple_struct("", len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_tuple_struct(variant, len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(map::Serializer::new(self.0))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(structure::Serializer::new(self.0, name))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(variant, len)
    }
}

/// Format value's debug representation into a given [`Formatter`].
pub fn fmt<T: ?Sized + Serialize>(value: &T, f: &mut Formatter) -> fmt::Result {
    debug(value).fmt(f)
}

/// Convert value into a string with a concise debug representation.
pub fn to_string<T: ?Sized + Serialize>(value: &T) -> String {
    format!("{:?}", debug(value))
}

/// Pretty-print value into a string with a debug representation.
pub fn to_string_pretty<T: ?Sized + Serialize>(value: &T) -> String {
    format!("{:#?}", debug(value))
}
