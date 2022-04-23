use serial_test::serial;

use crate::cpsc411;
use crate::imp_cmf_lang as target;
use crate::imp_mf_lang as source;

#[test]
#[serial]
fn basic_with_tail() {
    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 0 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 0 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_with_tail() {
    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![source::Effect::begin { effects: vec![] }],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_with_tail_with_sub_effect() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![source::Effect::begin {
                        effects: vec![source::Effect::set_aloc_value {
                            aloc: aloc.clone(),
                            value: source::Value::triv {
                                triv: source::Triv::int64 { int64: 0 },
                            },
                        }],
                    }],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![target::Effect::set_aloc_value {
                    aloc: aloc.clone(),
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 0 },
                    },
                }],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_with_tail_with_multiple_sub_effects() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![source::Effect::begin {
                        effects: vec![
                            source::Effect::set_aloc_value {
                                aloc: aloc.clone(),
                                value: source::Value::triv {
                                    triv: source::Triv::int64 { int64: 1 },
                                },
                            },
                            source::Effect::set_aloc_value {
                                aloc: aloc.clone(),
                                value: source::Value::triv {
                                    triv: source::Triv::int64 { int64: 2 },
                                },
                            },
                        ],
                    }],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: target::Value::triv {
                            triv: target::Triv::int64 { int64: 1 },
                        },
                    },
                    target::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: target::Value::triv {
                            triv: target::Triv::int64 { int64: 2 },
                        },
                    },
                ],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_binop_value() {
    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::binop_triv_triv {
                    binop: cpsc411::Binop::plus,
                    triv1: source::Triv::int64 { int64: 3 },
                    triv2: source::Triv::int64 { int64: 2 },
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::value {
                value: target::Value::binop_triv_triv {
                    binop: cpsc411::Binop::plus,
                    triv1: target::Triv::int64 { int64: 3 },
                    triv2: target::Triv::int64 { int64: 2 },
                },
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_begin_with_sub_effect() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![source::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: source::Value::triv {
                            triv: source::Triv::int64 { int64: 0 },
                        },
                    }],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![target::Effect::set_aloc_value {
                    aloc: aloc.clone(),
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 0 },
                    },
                }],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_begin_with_recursion_depth_1() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![source::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: source::Value::binop_triv_triv {
                            binop: cpsc411::Binop::plus,
                            triv1: source::Triv::int64 { int64: 0 },
                            triv2: source::Triv::int64 { int64: 0 },
                        },
                    }],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![target::Effect::set_aloc_value {
                    aloc: aloc.clone(),
                    // value: target::Value::triv {
                    //     triv: target::Triv::int64 { int64: 0 },
                    // },
                    value: target::Value::binop_triv_triv {
                        binop: cpsc411::Binop::plus,
                        triv1: target::Triv::int64 { int64: 0 },
                        triv2: target::Triv::int64 { int64: 0 },
                    },
                }],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_begin_with_recursion_depth_2() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::ImpMfLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::begin {
                    effects: vec![
                        source::Effect::set_aloc_value {
                            aloc: aloc.clone(),
                            value: source::Value::begin {
                                effects: vec![],
                                value: Box::new(
                                    source::Value::binop_triv_triv {
                                        binop: cpsc411::Binop::plus,
                                        triv1: source::Triv::int64 { int64: 2 },
                                        triv2: source::Triv::int64 { int64: 3 },
                                    },
                                ),
                            },
                        },
                        source::Effect::set_aloc_value {
                            aloc: aloc.clone(),
                            value: source::Value::begin {
                                effects: vec![],
                                value: Box::new(source::Value::triv {
                                    triv: source::Triv::int64 { int64: 9 },
                                }),
                            },
                        },
                    ],
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::int64 { int64: 10 },
                    }),
                },
            },
        },
    };

    let expected = target::ImpCmfLang {
        p: target::P::module {
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: target::Value::binop_triv_triv {
                            binop: cpsc411::Binop::plus,
                            triv1: target::Triv::int64 { int64: 2 },
                            triv2: target::Triv::int64 { int64: 3 },
                        },
                    },
                    target::Effect::set_aloc_value {
                        aloc: aloc.clone(),
                        value: target::Value::triv {
                            triv: target::Triv::int64 { int64: 9 },
                        },
                    },
                ],
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::int64 { int64: 10 },
                    },
                }),
            },
        },
    };

    cpsc411::reset_all_indices();

    let actual = program.normalize_bind();
    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}
