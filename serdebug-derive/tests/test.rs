extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serdebug;
#[macro_use]
extern crate serdebug_derive;

macro_rules! test {
	(@decl $(# $attr:tt)* struct { $($payload:tt)* }) => {
		#[derive(Default)]
		#[allow(dead_code)]
		$(# $attr)*
		pub struct Test { $($payload)* }
	};

	(@decl $(# $attr:tt)* struct $($payload:tt)*) => {
		#[derive(Default)]
		#[allow(dead_code)]
		$(# $attr)*
		pub struct Test $($payload)*;
	};

	(@decl $(# $attr:tt)* enum $($payload:tt)*) => {
		$(# $attr)*
		pub enum Test {
			#[allow(dead_code)]
			Variant $($payload)*
		}

		impl Default for Test {
			fn default() -> Self {
				test!(@decl struct $($payload)*);

				unsafe {
					::std::mem::transmute(Test::default())
				}
			}
		}
	};

	(@kind $name:ident $kind:tt $($payload:tt)*) => {
		#[test]
		fn $name() {
			mod lhs {
				test!(@decl #[derive(Serialize, SerDebug)] $kind $($payload)*);
			}

			mod rhs {
				test!(@decl #[derive(Debug)] $kind $($payload)*);
			}

			let lhs = lhs::Test::default();
			let rhs = rhs::Test::default();

			assert_eq!(format!("{:#?}", lhs), format!("{:#?}", rhs));
			assert_eq!(format!("{:?}", lhs), format!("{:?}", rhs));
		}
	};

	($name:ident $($payload:tt)*) => {
		mod $name {
			test!(@kind test_struct struct $($payload)*);
			test!(@kind test_enum enum $($payload)*);
		}
	};
}

test!(named_fields { a: u32, b: Option<f64>, c: &'static str, d: () });
test!(empty_named_fields {});
test!(tuple_fields(u32, Option<f64>, &'static str, ()));
test!(single_tuple_field(u32));
test!(empty_tuple_fields());
test!(no_fields);
