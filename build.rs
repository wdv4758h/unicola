extern crate unicola_codegen;

use unicola_codegen::generator::*;

fn main() {
    // using pre-generated code, fast return
    if cfg!(pregen) {
        return;
    }

    generate_unicode_version();

    if cfg!(feature="width") {
        generate_width_table();
    }

    if cfg!(feature="eac") {
        generate_emoji_codes();
    }
}
