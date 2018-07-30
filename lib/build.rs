extern crate rustc_version;

use rustc_version::{version_meta, Channel::Nightly, VersionMeta};

fn main() {
    if let Ok(VersionMeta {
        channel: Nightly, ..
    }) = version_meta()
    {
        println!("cargo:rustc-cfg=nightly");
    }
}
