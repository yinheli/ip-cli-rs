use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/*");
    copy_to_output("assets", &env::var("PROFILE").unwrap()).expect("Could not copy");
    copy_to_output("README.md", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
