extern crate serde;

use std::fmt::{self, Debug, DebugList, DebugMap, DebugTuple, DebugStruct, Formatter};
use serde::ser;

pub struct Serialize<T: ?Sized + ser::Serialize>(pub T);

impl<T: ?Sized + ser::Serialize> Debug for Serialize<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        ser::Serialize::serialize(&self.0, Serializer(f))?;
        Ok(())
    }
}

pub struct Serializer<'a, 'b: 'a>(pub &'a mut Formatter<'b>);

macro_rules! simple_impl {
    ($(fn $name:ident ( $self: ident, $v:ident : $ty:ty ) -> $res:ty;)*) => {
        $(fn $name($self, $v: $ty) -> $res {
            Ok($v.fmt($self.0)?)
        })*
    };
}

pub struct Error(fmt::Error);

impl From<Error> for fmt::Error {
    fn from(err: Error) -> fmt::Error {
        err.0
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error(err)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        unreachable!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        std::error::Error::description(&self.0)
    }

    fn cause(&self) -> Option<&std::error::Error> {
        Some(&self.0)
    }
}

impl<'a, 'b: 'a> ser::Serializer for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, 'b>;
    type SerializeTuple = TupleSerializer<'a, 'b>;
    type SerializeTupleStruct = TupleSerializer<'a, 'b>;
    type SerializeTupleVariant = TupleSerializer<'a, 'b>;
    type SerializeMap = MapSerializer<'a, 'b>;
    type SerializeStruct = StructSerializer<'a, 'b>;
    type SerializeStructVariant = StructSerializer<'a, 'b>;

    simple_impl! {
        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().fmt(self.0)?)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        ser::SerializeTupleStruct::end(self.serialize_tuple_struct(name, 0)?)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct(name)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct("None")
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        value: &T
    ) -> Result<Self::Ok, Self::Error> {
        let mut tuple = self.serialize_tuple_struct(name, 1)?;
        ser::SerializeTupleStruct::serialize_field(&mut tuple, value)?;
        ser::SerializeTupleStruct::end(tuple)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_struct(name, value)
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        value: &T
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_struct("Some", value)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer(self.0.debug_list()))
    }

    fn serialize_tuple(
        self,
        len: usize
    ) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_tuple_struct("", len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleSerializer(self.0.debug_tuple(name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_tuple_struct(name, len)
    }

    fn serialize_map(
        self,
        _len: Option<usize>
    ) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer(self.0.debug_map()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer(self.0.debug_struct(name)))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(name, len)
    }
}

pub struct SeqSerializer<'a, 'b: 'a>(DebugList<'a, 'b>);

impl<'a, 'b: 'a> ser::SerializeSeq for SeqSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.entry(&Serialize(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}

pub struct MapSerializer<'a, 'b: 'a>(DebugMap<'a, 'b>);

impl<'a, 'b: 'a> ser::SerializeMap for MapSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        unimplemented!("Only entries are supported in DebugMap")
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        unimplemented!("Only entries are supported in DebugMap")
    }

    fn serialize_entry<K: ?Sized + ser::Serialize, V: ?Sized + ser::Serialize>(&mut self, key: &K, value: &V) -> Result<(), Self::Error> {
        self.0.entry(&Serialize(key), &Serialize(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}

pub struct TupleSerializer<'a, 'b: 'a>(DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> ser::SerializeTuple for TupleSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.field(&Serialize(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}

impl<'a, 'b: 'a> ser::SerializeTupleStruct for TupleSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeTuple::end(self)
    }
}

impl<'a, 'b: 'a> ser::SerializeTupleVariant for TupleSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeTuple::end(self)
    }
}

pub struct StructSerializer<'a, 'b: 'a>(DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> ser::SerializeStructVariant for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeStruct::end(self)
    }
}

impl<'a, 'b: 'a> ser::SerializeStruct for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.0.field(key, &Serialize(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}
