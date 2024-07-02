use serial_test::serial;

use crate::imperative_abstractions::values_lang as source;
use crate::imperative_abstractions::values_unique_lang as target;
use crate::utils;

#[test]
#[serial]
fn book_example_1() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::binop_triv_triv {
            binop: utils::Binop::plus,
            triv1: source::Triv::int64(2),
            triv2: source::Triv::int64(2),
        }),
    });
    let actual = p.uniquify();
    // let expected = target::ValuesUniqueLang(target::P::module(
    //     target::Tail::value(target::Value::binop_triv_triv {
    //         binop: utils::Binop::plus,
    //         triv1: target::Triv::int64(2),
    //         triv2: target::Triv::int64(2),
    //     }),
    // ));
    // let actual = program.uniquify();
    // assert_eq!(actual, expected);
    // utils::reset_all_indices();
}

#[test]
#[serial]
fn book_example_2() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![(
                "x".into(),
                source::Value::triv(source::Triv::int64(5)),
            )]
            .into_iter()
            .collect::<_>(),
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::name("x".into()),
            ))),
        },
    });
    let actual = p.uniquify();
    // let aloc = utils::Aloc::fresh();
    // let expected =
    //     target::ValuesUniqueLang(target::P::module(target::Tail::r#let {
    //         bindings: vec![(
    //             aloc.clone(),
    //             target::Value::triv(target::Triv::int64(5)),
    //         )]
    //         .into_iter()
    //         .collect::<_>(),
    //         tail: Box::new(target::Tail::value(target::Value::triv(
    //             target::Triv::aloc(aloc),
    //         ))),
    //     }));
    // utils::reset_all_indices();
    // let actual = program.uniquify();
    // assert_eq!(actual, expected);
    // utils::reset_all_indices();
}

#[test]
#[serial]
fn same_scope_reference() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![
                ("a", source::Value::triv(source::Triv::int64(10))),
                ("b", source::Value::triv(source::Triv::name("a"))),
            ]
            .into_iter()
            .collect(),
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::name("a"),
            ))),
        },
    });
    let actual = p.uniquify();
}

#[test]
#[serial]
fn shadowing() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![
                ("a", source::Value::triv(source::Triv::int64(10))),
                ("a", source::Value::triv(source::Triv::int64(101))),
            ]
            .into_iter()
            .collect(),
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::name("a"),
            ))),
        },
    });
    let actual = p.uniquify();
}

#[test]
#[serial]
#[should_panic]
fn invalid_identifier() {
    let program = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![].into_iter().collect(),
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::name("x"),
            ))),
        },
    });
    program.uniquify();
}

#[test]
#[serial]
#[should_panic]
fn invalid_nested_identifier() {
    let program = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::r#let {
            bindings: vec![("z", source::Value::r#let {
                bindings: vec![(
                    "x",
                    source::Value::triv(source::Triv::int64(5)),
                )]
                .into_iter()
                .collect(),
                value: Box::new(source::Value::triv(source::Triv::name("x"))),
            })]
            .into_iter()
            .collect(),
            tail: Box::new(source::Tail::value(source::Value::triv(
                source::Triv::name("x"),
            ))),
        },
    });
    program.uniquify();
}
