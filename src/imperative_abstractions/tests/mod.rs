use serial_test::serial;

use crate::imperative_abstractions::compile;
use crate::imperative_abstractions::values_lang as source;
use crate::register_allocation::asm_pred_lang as target;
use crate::utils;

#[test]
#[serial]
fn basic() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::triv(source::Triv::int64(5))),
    });
    let actual = compile(p).unwrap();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::halt(target::Triv::int64(5)),
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn basic_if_condition() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#if {
            pred: source::Pred::relop {
                relop: utils::Relop::gt,
                triv1: source::Triv::int64(5),
                triv2: source::Triv::int64(6),
            },
            tail1: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::int64(7),
            ))),
            tail2: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::int64(8),
            ))),
        },
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let aloc = utils::Aloc::fresh();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![target::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: target::Triv::int64(5),
            }],
            tail: Box::new(target::Tail::r#if {
                pred: target::Pred::relop {
                    relop: utils::Relop::gt,
                    aloc,
                    triv: target::Triv::int64(6),
                },
                tail1: Box::new(target::Tail::halt(target::Triv::int64(7))),
                tail2: Box::new(target::Tail::halt(target::Triv::int64(8))),
            }),
        },
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn basic_operation() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::binop_triv_triv {
            binop: utils::Binop::plus,
            triv1: source::Triv::int64(10),
            triv2: source::Triv::int64(11),
        }),
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let aloc = utils::Aloc::fresh();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![
                target::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: target::Triv::int64(10),
                },
                target::Effect::set_aloc_binop_aloc_triv {
                    aloc: aloc.clone(),
                    binop: utils::Binop::plus,
                    triv: target::Triv::int64(11),
                },
            ],
            tail: Box::new(target::Tail::halt(target::Triv::aloc(aloc))),
        },
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn not_if_condition() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#if {
            pred: source::Pred::not(Box::new(source::Pred::relop {
                relop: utils::Relop::gt,
                triv1: source::Triv::int64(5),
                triv2: source::Triv::int64(6),
            })),
            tail1: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::int64(7),
            ))),
            tail2: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::int64(8),
            ))),
        },
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let aloc = utils::Aloc::fresh();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![target::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: target::Triv::int64(5),
            }],
            tail: Box::new(target::Tail::r#if {
                pred: target::Pred::not(Box::new(target::Pred::relop {
                    relop: utils::Relop::gt,
                    aloc,
                    triv: target::Triv::int64(6),
                })),
                tail1: Box::new(target::Tail::halt(target::Triv::int64(7))),
                tail2: Box::new(target::Tail::halt(target::Triv::int64(8))),
            }),
        },
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn empty_bindings_in_tail() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![],
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::int64(5),
            ))),
        },
    });
    let actual = compile(p).unwrap();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::halt(target::Triv::int64(5)),
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn nested_empty_bindings_in_tail() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![],
            tail: Box::new(source::Tail::r#let {
                bindings: vec![],
                tail: Box::new(source::Tail::value(source::Value::triv(
                    source::Triv::int64(5),
                ))),
            }),
        },
    });
    let actual = compile(p).unwrap();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::halt(target::Triv::int64(5)),
    });
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn nested_if_condition_in_tail() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![],
            tail: Box::new(source::Tail::r#let {
                bindings: vec![],
                tail: Box::new(source::Tail::r#if {
                    pred: source::Pred::relop {
                        relop: utils::Relop::eq,
                        triv1: source::Triv::int64(0),
                        triv2: source::Triv::int64(0),
                    },
                    tail1: Box::new(source::Tail::value(source::Value::triv(
                        source::Triv::int64(1),
                    ))),
                    tail2: Box::new(source::Tail::value(source::Value::triv(
                        source::Triv::int64(2),
                    ))),
                }),
            }),
        },
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let aloc = utils::Aloc::fresh();
    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![target::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: target::Triv::int64(0),
            }],
            tail: Box::new(target::Tail::r#if {
                pred: target::Pred::relop {
                    relop: utils::Relop::eq,
                    aloc,
                    triv: target::Triv::int64(0),
                },
                tail1: Box::new(target::Tail::halt(target::Triv::int64(1))),
                tail2: Box::new(target::Tail::halt(target::Triv::int64(2))),
            }),
        },
    });
    assert_eq!(actual, expected);
}
