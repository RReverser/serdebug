extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().unwrap().channel {
        Channel::Nightly | Channel::Dev => {
            println!("cargo:rustc-cfg=nightly");
        }
        _ => {}
    }
}
