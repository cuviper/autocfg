#![allow(unknown_lints)]
#![deny(unexpected_cfgs)]

macro_rules! test_cfgs {
    ($($cfg:ident,)*) => {$({
        let cfg_desc = format!("cfg!({})", stringify!($cfg));
        if cfg!($cfg) {
            println!("Enabled:    {}", cfg_desc);
        } else {
            println!("Disabled:   {}", cfg_desc);
        }
    })*};

}

pub fn main() {
    test_cfgs!(
        // emit_rustc_version
        rustc_1_0,
        rustc_7_4294967295,
        // emit_has_path, emit_path_cfg
        has_std__vec__Vec,
        has_path_std_vec,
        has_dummy__DummyPath,
        has_path_dummy,
        // emit_has_trait, emit_trait_cfg
        has_std__ops__Add,
        has_trait_add,
        has_dummy__DummyTrait,
        has_trait_dummy,
        // emit_has_type, has_type_i32
        has_i32,
        has_type_i32,
        has_i7billion,
        has_type_i7billion,
        // emit_expression_cfg
        has_working_addition,
        has_working_5xor,
        // emit_constant_cfg
        has_const_7,
        has_const_file_open,
    );
}
