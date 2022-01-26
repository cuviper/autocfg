extern crate autocfg;

use std::env;

/// Tests that autocfg uses the RUSTFLAGS environment variable when running
/// rustc.
#[test]
fn test_with_sysroot_rustflags() {
    test_with_sysroot("RUSTFLAGS")
}

/// Tests that autocfg uses the CARGO_ENCODED_RUSTFLAGS environment
/// variable when running rustc.
#[test]
fn test_with_sysroot_cargo_encoded_rustflags() {
    test_with_sysroot("CARGO_ENCODED_RUSTFLAGS")
}

fn test_with_sysroot(var_name: &str) {
    // Use the same path as this test binary.
    let dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    env::set_var(var_name, &format!("-L {}", dir.display()));
    env::set_var("OUT_DIR", &format!("{}", dir.display()));

    // Ensure HOST != TARGET.
    env::set_var("HOST", "lol");

    let ac = autocfg::AutoCfg::new().unwrap();
    assert!(ac.probe_sysroot_crate("autocfg"));
}
