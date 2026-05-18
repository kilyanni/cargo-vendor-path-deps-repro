// Mimics substrate-wasm-builder: generate a tiny stub crate that
// path-depends on this crate's CARGO_MANIFEST_DIR, then spawn a nested
// `cargo build` on it. Inside that nested invocation, cargo reaches
// crate-a as a path source, so it follows crate-a's own `path = "..."`
// declarations literally (the lock doesn't override sub-dep sources when
// the parent is a path source). That's where the broken
// `crate-b = { path = "../crate-b" }` declaration ENOENTs.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // The nested cargo will rebuild crate-a as a dep, which would run this
    // build script again — break the recursion. (In the bug-repro scenario
    // this never fires because the nested cargo dies during resolution.)
    if env::var("WBUILD_NESTED").is_ok() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo = env::var("CARGO").unwrap();

    let wbuild = out_dir.join("wbuild");
    fs::create_dir_all(wbuild.join("src")).unwrap();

    let toml = format!(
        r#"[package]
name = "wbuild-stub"
version = "0.1.0"
edition = "2021"

[dependencies]
crate-a = {{ path = "{manifest_dir}" }}

# Prevent cargo from auto-discovering an enclosing workspace.
[workspace]
"#
    );
    fs::write(wbuild.join("Cargo.toml"), toml).unwrap();
    fs::write(wbuild.join("src/lib.rs"), "").unwrap();

    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg(wbuild.join("Cargo.toml"))
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC")
        .env("WBUILD_NESTED", "1")
        .env("CARGO_TARGET_DIR", out_dir.join("wbuild-target"))
        .status()
        .expect("failed to spawn nested cargo");

    assert!(
        status.success(),
        "nested cargo build failed — this is the fetchCargoVendor path-dep bug"
    );
}
