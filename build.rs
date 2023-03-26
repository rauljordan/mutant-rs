use std::env;
use std::path::Path;

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir);

    env::set_current_dir(path.join("zig")).unwrap();

    Command::new("zig")
        .args(&["build", "-Doptimize=ReleaseFast"])
        .output()
        .expect("Failed to compile Zig lib");

    env::set_current_dir(path).unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("zig/zig-out/lib").display()
    );
}
