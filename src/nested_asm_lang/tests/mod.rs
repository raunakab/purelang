use serial_test::serial;

use crate::nested_asm_lang as source;

#[test]
#[serial]
fn basic() {
    let program = source::NestedAsmLang {
        p: source::P::module { tail: source::Tail::halt { triv: source::Triv::int64 { int64: 5 } } },
    };

    let result = program
        .optimize_predicates()
        .expose_basic_blocks()
        .resolve_predicates()
        .flatten_program()
        .patch_instructions()
        .implement_fvars()
        .generate_x64();

    println!("ya: {}\n done", result);

    panic!();
}
