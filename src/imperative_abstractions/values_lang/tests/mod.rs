use serial_test::serial;

use crate::utils;
use crate::imperative_abstractions::values_lang as source;
use crate::imperative_abstractions::values_unique_lang as target;

#[test]
#[serial]
fn book_example_1() {
    let program = source::ValuesLang(source::P::module(source::Tail::value(
        source::Value::binop_triv_triv {
            binop: utils::Binop::plus,
            triv1: source::Triv::int64(2),
            triv2: source::Triv::int64(2),
        },
    )));

    let expected = target::ValuesUniqueLang {
        p: target::P::module {
            tail: target::Tail::value {
                value: target::Value::binop_triv_triv {
                    binop: utils::Binop::plus,
                    triv1: target::Triv::int64 { int64: 2 },
                    triv2: target::Triv::int64 { int64: 2 },
                },
            },
        },
    };

    let actual = program.uniquify();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}

#[test]
#[serial]
fn book_example_2() {
    let program = source::ValuesLang(source::P::module(source::Tail::r#let {
        bindings: vec![(
            "x".into(),
            source::Value::triv(source::Triv::int64(5)),
        )]
        .into_iter()
        .collect::<_>(),
        tail: Box::new(source::Tail::value(source::Value::triv(
            source::Triv::name("x".into()),
        ))),
    }));

    let aloc = utils::Aloc::fresh();

    let expected = target::ValuesUniqueLang {
        p: target::P::module {
            tail: target::Tail::r#let {
                bindings: vec![(aloc.clone(), target::Value::triv {
                    triv: target::Triv::int64 { int64: 5 },
                })]
                .into_iter()
                .collect::<_>(),
                tail: Box::new(target::Tail::value {
                    value: target::Value::triv {
                        triv: target::Triv::aloc { aloc },
                    },
                }),
            },
        },
    };

    utils::reset_all_indices();

    let actual = program.uniquify();

    assert_eq!(actual, expected);

    utils::reset_all_indices();
}
