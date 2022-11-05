use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/*");
    let build_type = &env::var("PROFILE").unwrap();
    copy_to_output("assets", build_type).expect("Could not copy");
    copy_to_output("README.md", build_type).expect("Could not copy");
}
