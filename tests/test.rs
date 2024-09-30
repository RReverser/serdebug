use serde::Serialize;
use serdebug::SerDebug;
use std::collections::HashMap;
use std::fmt::Debug;
use test_strategy::{proptest, Arbitrary};

fn test_via_serdebug<T: Debug + Serialize>(lhs: T) {
    #[derive(Serialize, SerDebug)]
    #[serde(transparent)]
    struct Rhs<'a, T>(&'a T);

    let rhs = Rhs(&lhs);

    assert_eq!(format!("{lhs:#?}"), format!("{rhs:#?}"));
    assert_eq!(format!("{lhs:?}"), format!("{rhs:?}"));
}

macro_rules! test {
	(@decl $(# $attr:tt)* struct { $($payload:tt)* }) => {
		$(# $attr)*
		struct Lhs { $($payload)* }
	};

	(@decl $(# $attr:tt)* struct $($payload:tt)*) => {
		$(# $attr)*
		struct Lhs $($payload)*;
	};

	(@decl $(# $attr:tt)* enum $($payload:tt)*) => {
		$(# $attr)*
		enum Lhs {
			Variant $($payload)*
		}
	};

	// Uninhabited enums can't be constructed by definition.
	(@kind $name:ident enum {}) => {};

	(@kind $name:ident $kind:ident $($payload:tt)*) => {
		mod $name {
			use crate::*;

			test!(@decl #[derive(Serialize, Debug, Arbitrary)] $kind $($payload)*);

			#[proptest]
			fn test(lhs: Lhs) {
				test_via_serdebug(lhs);
			}
		}
	};

	($name:ident $($payload:tt)*) => {
		mod $name {
			test!(@kind test_struct struct $($payload)*);
			test!(@kind test_enum enum $($payload)*);
		}
	};
}

test!(named_fields { a: u32, b: Option<f64>, c: String, d: (), e: HashMap<u32, String>, f: Vec<u32> });
test!(empty_named_fields {});
test!(tuple_fields(u32, Option<f64>, String, (), HashMap<u32, String>, Vec<u32>));
test!(single_tuple_field(u32));
test!(empty_tuple_fields());
test!(no_fields);
