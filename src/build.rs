use fs_extra::{copy_items, dir::CopyOptions};
use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=assets/*");
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = Path::new(&out_dir).join(Path::new("../../../"));
    let target = target.to_str().unwrap();
    // println!("cargo:warning=target: {target:?}");
    let mut options = CopyOptions::default();
    options.overwrite = true;
    copy_items(&["assets", "README.md"], target, &options).expect("copy failed");
}
