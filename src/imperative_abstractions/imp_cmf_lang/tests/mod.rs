use serial_test::serial;

use crate::imperative_abstractions::imp_cmf_lang as source;
use crate::register_allocation::asm_pred_lang as target;
use crate::utils;

#[test]
#[serial]
fn book_example_1() {
    let program = source::ImpCmfLang(source::P::module(source::Tail::value(
        source::Value::binop_triv_triv {
            binop: utils::Binop::plus,
            triv1: source::Triv::int64(2),
            triv2: source::Triv::int64(2),
        },
    )));

    let tmp = utils::Aloc::fresh();

    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![
                target::Effect::set_aloc_triv {
                    aloc: tmp.clone(),
                    triv: target::Triv::int64(2),
                },
                target::Effect::set_aloc_binop_aloc_triv {
                    aloc: tmp.clone(),
                    binop: utils::Binop::plus,
                    triv: target::Triv::int64(2),
                },
            ],
            tail: Box::new(target::Tail::halt(target::Triv::aloc(tmp))),
        },
    });

    utils::reset_all_indices();

    let actual = program.select_instructions();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}

#[test]
#[serial]
fn book_example_2() {
    let aloc = utils::Aloc::fresh_with_name("x");
    let program = source::ImpCmfLang(source::P::module(source::Tail::begin {
        effects: vec![source::Effect::set_aloc_value {
            aloc: aloc.clone(),
            value: source::Value::triv(source::Triv::int64(5)),
        }],
        tail: Box::new(source::Tail::value(source::Value::triv(
            source::Triv::aloc(aloc.clone()),
        ))),
    }));

    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![target::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: target::Triv::int64(5),
            }],
            tail: Box::new(target::Tail::halt(target::Triv::aloc(aloc))),
        },
    });

    let actual = program.select_instructions();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}

#[test]
#[serial]
fn book_example_3() {
    let aloc = utils::Aloc::fresh_with_name("x");
    let program = source::ImpCmfLang(source::P::module(source::Tail::begin {
        effects: vec![source::Effect::set_aloc_value {
            aloc: aloc.clone(),
            value: source::Value::binop_triv_triv {
                binop: utils::Binop::plus,
                triv1: source::Triv::int64(2),
                triv2: source::Triv::int64(2),
            },
        }],
        tail: Box::new(source::Tail::value(source::Value::triv(
            source::Triv::aloc(aloc.clone()),
        ))),
    }));

    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![
                target::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: target::Triv::int64(2),
                },
                target::Effect::set_aloc_binop_aloc_triv {
                    aloc: aloc.clone(),
                    binop: utils::Binop::plus,
                    triv: target::Triv::int64(2),
                },
            ],
            tail: Box::new(target::Tail::halt(target::Triv::aloc(aloc))),
        },
    });

    let actual = program.select_instructions();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}

#[test]
#[serial]
fn book_example_4() {
    let aloc1 = utils::Aloc::fresh_with_name("x");
    let aloc2 = utils::Aloc::fresh_with_name("x");

    let program = source::ImpCmfLang(source::P::module(source::Tail::begin {
        effects: vec![
            source::Effect::set_aloc_value {
                aloc: aloc1.clone(),
                value: source::Value::triv(source::Triv::int64(2)),
            },
            source::Effect::set_aloc_value {
                aloc: aloc2.clone(),
                value: source::Value::triv(source::Triv::int64(2)),
            },
        ],
        tail: Box::new(source::Tail::value(source::Value::binop_triv_triv {
            binop: utils::Binop::plus,
            triv1: source::Triv::aloc(aloc1.clone()),
            triv2: source::Triv::aloc(aloc1.clone()),
        })),
    }));

    let tmp = utils::Aloc {
        name: "tmp".into(),
        index: 2,
    };

    let expected = target::AsmPredLang(target::P::module {
        info: utils::Info::default(),
        tail: target::Tail::begin {
            effects: vec![
                target::Effect::set_aloc_triv {
                    aloc: aloc1.clone(),
                    triv: target::Triv::int64(2),
                },
                target::Effect::set_aloc_triv {
                    aloc: aloc2.clone(),
                    triv: target::Triv::int64(2),
                },
                target::Effect::set_aloc_triv {
                    aloc: tmp.clone(),
                    triv: target::Triv::aloc(aloc1.clone()),
                },
                target::Effect::set_aloc_binop_aloc_triv {
                    aloc: tmp.clone(),
                    binop: utils::Binop::plus,
                    triv: target::Triv::aloc(aloc2.clone()),
                },
            ],
            tail: Box::new(target::Tail::halt(target::Triv::aloc(tmp))),
        },
    });

    let actual = program.select_instructions();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}
