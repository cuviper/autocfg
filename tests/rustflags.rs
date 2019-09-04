extern crate autocfg;

/// Tests that autocfg uses the RUSTFLAGS environment variable when running
/// rustc.
#[test]
fn test_with_sysroot() {
    std::env::set_var("RUSTFLAGS", "-L target/debug/deps");
    std::env::set_var("OUT_DIR", "target");
    let ac = autocfg::AutoCfg::new().unwrap();
    assert!(ac.probe_sysroot_crate("autocfg"));
}