use crate::asm_lang as source;
use crate::cpsc411;

#[test]
fn basic() {
    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::halt {
                triv: source::Triv::int64 { int64: 0 },
            },
        },
    };

    let ir = crate::purelang_c(program);
    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov rax, 0"
            .to_string(),
    );

    assert_eq!(result, 0,);
}

#[test]
fn one_basic_effect() {
    let program = source::AsmLang {
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
    };

    let ir = crate::purelang_c(program);
    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 22
mov rax, 2005"
            .to_string()
    );

    assert_eq!(result, 2005,);
}

#[test]
fn multiple_basic_effects() {
    let program = source::AsmLang {
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
    };

    let ir = crate::purelang_c(program);
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

    assert_eq!(result, 1973,);
}

#[test]
fn one_complex_effect() {
    let aloc_1 = cpsc411::Aloc::fresh();
    let aloc_2 = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
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
    };

    let ir = crate::purelang_c(program);
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
}

#[test]
fn one_complex_effect_with_addition() {
    let aloc_1 = cpsc411::Aloc::fresh();
    let aloc_2 = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
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
    };

    let ir = crate::purelang_c(program);
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
}
