use serial_test::serial;

use crate::asm_lang as source;
use crate::cpsc411;
use crate::cpsc411::Compile;
use crate::cpsc411::Interpret;

#[test]
#[serial]
fn basic() {
    let ir = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::halt {
                triv: source::Triv::int64 { int64: 0 },
            },
        },
    }
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov rax, 0"
            .to_string(),
    );

    assert_eq!(result, 0);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn one_basic_effect() {
    let ir = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_triv {
                    aloc: cpsc411::Aloc::fresh(),
                    triv: source::Triv::int64 { int64: 22 },
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::int64 { int64: 2005 },
                }),
            },
        },
    }
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 22
mov rax, 2005"
            .to_string()
    );

    assert_eq!(result, 2005);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn multiple_basic_effects() {
    let ir = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_triv {
                        aloc: cpsc411::Aloc::fresh(),
                        triv: source::Triv::int64 { int64: 1220 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: cpsc411::Aloc::fresh(),
                        triv: source::Triv::int64 { int64: 1969 },
                    },
                ],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::int64 { int64: 1973 },
                }),
            },
        },
    }
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 1220
mov QWORD [rbp - 8], 1969
mov rax, 1973"
            .to_string()
    );

    assert_eq!(result, 1973);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn one_complex_effect() {
    let aloc_1 = cpsc411::Aloc::fresh();
    let aloc_2 = cpsc411::Aloc::fresh();

    let ir = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_triv {
                        aloc: aloc_1.clone(),
                        triv: source::Triv::int64 { int64: 1220 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: aloc_2.clone(),
                        triv: source::Triv::aloc {
                            aloc: aloc_1.clone(),
                        },
                    },
                ],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc {
                        aloc: aloc_1.clone(),
                    },
                }),
            },
        },
    }
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 1220
mov r10, QWORD [rbp - 0]
mov QWORD [rbp - 8], r10
mov rax, QWORD [rbp - 0]"
            .to_string()
    );

    assert_eq!(result, 1220);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn one_complex_effect_with_addition() {
    let aloc_1 = cpsc411::Aloc::fresh();
    let aloc_2 = cpsc411::Aloc::fresh();

    let ir = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_triv {
                        aloc: aloc_1.clone(),
                        triv: source::Triv::int64 { int64: 10 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: aloc_2.clone(),
                        triv: source::Triv::int64 { int64: 20 },
                    },
                    source::Effect::set_aloc_binop_aloc_triv {
                        aloc: aloc_2.clone(),
                        binop: cpsc411::Binop::plus,
                        triv: source::Triv::aloc {
                            aloc: aloc_1.clone(),
                        },
                    },
                ],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc {
                        aloc: aloc_2.clone(),
                    },
                }),
            },
        },
    }
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 10
mov QWORD [rbp - 8], 20
mov r10, QWORD [rbp - 8]
mov r11, QWORD [rbp - 0]
add r10, r11
mov QWORD [rbp - 8], r10
mov rax, QWORD [rbp - 8]"
            .to_string()
    );

    assert_eq!(result, 30);

    cpsc411::reset_all_indices();
}
