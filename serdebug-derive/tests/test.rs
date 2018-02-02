extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serdebug;
#[macro_use] extern crate serdebug_derive;

macro_rules! struct_tests {
	(@struct ( $($payload:tt)* )) => {
		#[derive(Serialize, SerDebug, Default)]
		pub struct Test ( $($payload)* );
	};

	(@struct $payload:tt) => {
		#[derive(Serialize, SerDebug, Default)]
		pub struct Test $payload
	};

	($($payload:tt)*) => {
		$({
			mod serdebug {
				struct_tests!(@struct $payload);
			}

			mod debug {
				struct_tests!(@struct $payload);
			}

			assert_eq!(format!("{:?}", serdebug::Test::default()), format!("{:?}", debug::Test::default()));
			assert_eq!(format!("{:#?}", serdebug::Test::default()), format!("{:#?}", debug::Test::default()));
		})*
	};
}

#[test]
fn tests() {
	struct_tests! {
		{ a: u32, b: Option<f64> }
		(u32)
		(())
		()
		;
	}
}
