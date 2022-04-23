use serial_test::serial;

use crate::cpsc411;
use crate::values_lang as source;
use crate::values_unique_lang as target;

#[test]
#[serial]
fn book_example_1() {
    let program = source::ValuesLang {
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

    let expected = target::ValuesUniqueLang {
        p: target::P::module {
            tail: target::Tail::value {
                value: target::Value::binop_triv_triv {
                    binop: cpsc411::Binop::plus,
                    triv1: target::Triv::int64 { int64: 2 },
                    triv2: target::Triv::int64 { int64: 2 },
                },
            },
        },
    };

    let actual = program.uniquify();

    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn book_example_2() {
    let program = source::ValuesLang {
        p: source::P::module {
            tail: source::Tail::r#let {
                bindings: vec![("x".into(), source::Value::triv {
                    triv: source::Triv::int64 { int64: 5 },
                })]
                .into_iter()
                .collect::<_>(),
                tail: Box::new(source::Tail::value {
                    value: source::Value::triv {
                        triv: source::Triv::name { name: "x".into() },
                    },
                }),
            },
        },
    };

    let aloc = cpsc411::Aloc::fresh();

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

    cpsc411::reset_all_indices();

    let actual = program.uniquify();

    assert_eq!(actual, expected);

    cpsc411::reset_all_indices();
}
