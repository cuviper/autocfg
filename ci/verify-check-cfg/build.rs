extern crate autocfg;

pub fn main() {
    let cfg = autocfg::AutoCfg::new().unwrap();

    //
    // tests
    //

    // always true
    cfg.emit_rustc_version(1, 0);
    // should always be false
    cfg.emit_rustc_version(7, std::u32::MAX as usize);

    // always true
    cfg.emit_has_path("std::vec::Vec");
    cfg.emit_path_cfg("std::vec::Vec", "has_path_std_vec");
    // always false
    cfg.emit_has_path("dummy::DummyPath");
    cfg.emit_path_cfg("dummy::DummyPath", "has_path_dummy");

    // always true
    cfg.emit_has_trait("std::ops::Add");
    cfg.emit_trait_cfg("std::ops::Add", "has_trait_add");
    // always false
    cfg.emit_has_trait("dummy::DummyTrait");
    cfg.emit_trait_cfg("dummy::DummyTrait", "has_trait_dummy");

    // always true
    cfg.emit_has_type("i32");
    cfg.emit_type_cfg("i32", "has_type_i32");
    // always false
    cfg.emit_has_type("i7billion");
    cfg.emit_type_cfg("i7billion", "has_type_i7billion");

    // always true
    cfg.emit_expression_cfg("3 + 7", "has_working_addition");
    // always false
    cfg.emit_expression_cfg("3 ^^^^^ 12", "has_working_5xor");

    // always true
    cfg.emit_constant_cfg("7", "has_const_7");
    // false - Opening file should never be `const`
    cfg.emit_constant_cfg("std::fs::File::open(\"foo.txt\")", "has_const_file_open");
}
