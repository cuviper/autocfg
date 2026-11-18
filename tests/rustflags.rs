extern crate autocfg;

use std::env;
use std::path::Path;
use std::process::Command;

mod support;

fn check_sysroot(sysroot: &Path, expected: bool) {
    let out = support::out_dir();
    let check = |expected| {
        let ac = autocfg::AutoCfg::with_dir(out.as_ref()).unwrap();
        assert_eq!(ac.probe_sysroot_crate("std"), expected);
    };

    // If we have encoded rustflags, they take precedence, even if empty.
    env::set_var("CARGO_ENCODED_RUSTFLAGS", "");
    env::set_var("RUSTFLAGS", "--sysroot /not/a/sysroot");
    check(true);

    // Now try again with useful encoded args.
    env::set_var(
        "CARGO_ENCODED_RUSTFLAGS",
        &format!("--sysroot\x1f{}", sysroot.display()),
    );
    check(expected);

    // Try the old-style RUSTFLAGS, ensuring HOST != TARGET.
    env::remove_var("CARGO_ENCODED_RUSTFLAGS");
    env::set_var("HOST", "lol");
    check(false);

    env::set_var("RUSTFLAGS", &format!("--sysroot {}", sysroot.display()));
    check(expected);
}

/// Tests that autocfg uses the RUSTFLAGS or CARGO_ENCODED_RUSTFLAGS
/// environment variables when running rustc.
#[test]
fn test_with_sysroot() {
    // This should not be a valid sysroot.
    check_sysroot(&support::exe_dir(), false);

    // Now use the real sysroot from the same `rustc` that `AutoCfg` should be using.
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(rustc)
        .args(&["--print", "sysroot"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let sysroot = std::str::from_utf8(&output.stdout).unwrap().trim();

    check_sysroot(Path::new(sysroot), true);
}
