use crate::asm_lang as target;
use crate::cpsc411;
use crate::imp_cmf_lang as source;

#[test]
fn book_example_1() {
    let program = source::ImpCmfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::binop_triv_triv {
                    binop: cpsc411::Binop::plus,
                    triv1: source::Triv::int64 { int64: 2 },
                    triv2: source::Triv::int64 { int64: 2 },
                },
            },
        },
    };

    let tmp = cpsc411::Aloc::fresh();
    let expected = target::AsmLang {
        p: target::P::module {
            info: cpsc411::Info::default(),
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set_aloc_triv {
                        aloc: tmp.clone(),
                        triv: target::Triv::int64 { int64: 2 },
                    },
                    target::Effect::set_aloc_binop_aloc_triv {
                        aloc: tmp.clone(),
                        binop: cpsc411::Binop::plus,
                        triv: target::Triv::int64 { int64: 2 },
                    },
                ],
                tail: Box::new(target::Tail::halt {
                    triv: target::Triv::aloc { aloc: tmp },
                }),
            },
        },
    };

    cpsc411::Aloc::reset_index();

    let actual = program.select_instructions();

    assert_eq!(actual, expected);
}

#[test]
fn book_example_2() {
    let aloc = cpsc411::Aloc::fresh_with_name("x");
    let program = source::ImpCmfLang {
        p: source::P::module {
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_value {
                    aloc: aloc.clone(),
                    value: source::Value::triv {
                        triv: source::Triv::int64 { int64: 5 },
                    },
                }],
                tail: Box::new(source::Tail::value {
                    value: source::Value::triv {
                        triv: source::Triv::aloc { aloc: aloc.clone() },
                    },
                }),
            },
        },
    };

    let expected = target::AsmLang {
        p: target::P::module {
            info: cpsc411::Info::default(),
            tail: target::Tail::begin {
                effects: vec![target::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: target::Triv::int64 { int64: 5 },
                }],
                tail: Box::new(target::Tail::halt {
                    triv: target::Triv::aloc { aloc },
                }),
            },
        },
    };

    let actual = program.select_instructions();

    assert_eq!(actual, expected);
}

#[test]
fn book_example_3() {
    let aloc = cpsc411::Aloc::fresh_with_name("x");
    let program = source::ImpCmfLang {
        p: source::P::module {
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_value {
                    aloc: aloc.clone(),
                    value: source::Value::binop_triv_triv {
                        binop: cpsc411::Binop::plus,
                        triv1: source::Triv::int64 { int64: 2 },
                        triv2: source::Triv::int64 { int64: 2 },
                    },
                }],
                tail: Box::new(source::Tail::value {
                    value: source::Value::triv {
                        triv: source::Triv::aloc { aloc: aloc.clone() },
                    },
                }),
            },
        },
    };

    let expected = target::AsmLang {
        p: target::P::module {
            info: cpsc411::Info::default(),
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: target::Triv::int64 { int64: 2 },
                    },
                    target::Effect::set_aloc_binop_aloc_triv {
                        aloc: aloc.clone(),
                        binop: cpsc411::Binop::plus,
                        triv: target::Triv::int64 { int64: 2 },
                    },
                ],
                tail: Box::new(target::Tail::halt {
                    triv: target::Triv::aloc { aloc },
                }),
            },
        },
    };

    let actual = program.select_instructions();

    assert_eq!(actual, expected);
}

#[test]
fn book_example_4() {
    let aloc1 = cpsc411::Aloc::fresh_with_name("x");
    let aloc2 = cpsc411::Aloc::fresh_with_name("x");

    let program = source::ImpCmfLang {
        p: source::P::module {
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_value {
                        aloc: aloc1.clone(),
                        value: source::Value::triv {
                            triv: source::Triv::int64 { int64: 2 },
                        },
                    },
                    source::Effect::set_aloc_value {
                        aloc: aloc2.clone(),
                        value: source::Value::triv {
                            triv: source::Triv::int64 { int64: 2 },
                        },
                    },
                ],
                tail: Box::new(source::Tail::value {
                    value: source::Value::binop_triv_triv {
                        binop: cpsc411::Binop::plus,
                        triv1: source::Triv::aloc {
                            aloc: aloc1.clone(),
                        },
                        triv2: source::Triv::aloc {
                            aloc: aloc2.clone(),
                        },
                    },
                }),
            },
        },
    };

    let tmp = cpsc411::Aloc {
        name: "tmp".into(),
        index: 2,
    };

    let expected = target::AsmLang {
        p: target::P::module {
            info: cpsc411::Info::default(),
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set_aloc_triv {
                        aloc: aloc1.clone(),
                        triv: target::Triv::int64 { int64: 2 },
                    },
                    target::Effect::set_aloc_triv {
                        aloc: aloc2.clone(),
                        triv: target::Triv::int64 { int64: 2 },
                    },
                    target::Effect::set_aloc_triv {
                        aloc: tmp.clone(),
                        triv: target::Triv::aloc {
                            aloc: aloc1.clone(),
                        },
                    },
                    target::Effect::set_aloc_binop_aloc_triv {
                        aloc: tmp.clone(),
                        binop: cpsc411::Binop::plus,
                        triv: target::Triv::aloc {
                            aloc: aloc2.clone(),
                        },
                    },
                ],
                tail: Box::new(target::Tail::halt {
                    triv: target::Triv::aloc { aloc: tmp },
                }),
            },
        },
    };

    let actual = program.select_instructions();

    assert_eq!(actual, expected);
}
