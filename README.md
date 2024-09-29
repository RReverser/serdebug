# serdebug

[![Crates.io](https://img.shields.io/crates/v/serdebug.svg)](https://crates.io/crates/serdebug)
[![docs.rs](https://docs.rs/serdebug/badge.svg)](https://docs.rs/serdebug)

This is a drop-in replacement for `#[derive(Debug)]` that uses `serde::Serialize` under the hood to provide advanced control over output serialisation.

## Usage

By default, the generated code will produce exactly same output as `#[derive(Debug)]` for compatibility.

However, this might be not very interesting, so let's add some serde attributes to see how we can control debug representation:

```rust
use serde::Serialize;
use serdebug::SerDebug;

pub struct CustomType(u32);

#[derive(Serialize, SerDebug)]
pub enum MyEnum {
    // renaming items works as expected
    #[serde(rename = "AAAAAAA!!!")]
    A,

    B(u32),

    C { flag: bool },
}

#[derive(Serialize, SerDebug)]
// so does bulk rename on containers
#[serde(rename_all = "PascalCase")]
pub struct MyStruct {
    number: u32,

    my_enum: Vec<MyEnum>,

    // we might want to hide some items from the output
    #[serde(skip_serializing)]
    hidden: bool,

    // or override serialisation for otherwise verbose wrappers or
    // third-party types that don't implement `Debug` and/or `Serialize`
    #[serde(serialize_with = "custom_serialize")]
    custom_type: CustomType,
}

fn custom_serialize<S: serde::Serializer>(value: &CustomType, ser: S) -> Result<S::Ok, S::Error> {
    use serde::Serialize;

    value.0.serialize(ser)
}

fn main() {
    let s = MyStruct {
        number: 42,
        my_enum: vec![MyEnum::A, MyEnum::B(10), MyEnum::C { flag: true }],
        hidden: true,
        custom_type: CustomType(20),
    };

    assert_eq!(format!("{:#?}", s), "
MyStruct {
    Number: 42,
    MyEnum: [
        AAAAAAA!!!,
        B(
            10,
        ),
        C {
            flag: true,
        },
    ],
    CustomType: 20,
}
".trim());
}
```
