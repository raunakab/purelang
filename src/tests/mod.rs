use serial_test::serial;

use crate::compile;
use crate::imperative_abstractions::values_lang as source;
use crate::utils;

#[test]
#[serial]
fn basic() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::triv(source::Triv::int64(9))),
    });
    let actual = compile(p).unwrap();
    let expected = "L.main.0:
\tmov rax, 9
\tjmp L.done
L.done:
";
    assert_eq!(actual, expected,);
}

#[test]
#[serial]
fn let_bindings_basic() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::r#let {
            bindings: vec![(
                "x".into(),
                source::Value::triv(source::Triv::int64(100)),
            )]
            .into_iter()
            .collect(),
            value: Box::new(source::Value::triv(source::Triv::name(
                "x".into(),
            ))),
        }),
    });
    let actual = compile(p).unwrap();
    let expected = "L.main.0:
\tmov r9, 100
\tmov rax, r9
\tjmp L.done
L.done:
";
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn let_bindings_optimzed() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::triv(source::Triv::int64(
            100,
        ))),
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let expected = "L.main.0:
\tmov rax, 100
\tjmp L.done
L.done:
";
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn let_bindings_with_operation_basic() {
    let p = source::ValuesLang(source::P::module {
        lambdas: vec![],
        tail: source::Tail::value(source::Value::r#let {
            bindings: vec![("x".into(), source::Value::binop_triv_triv {
                binop: utils::Binop::plus,
                triv1: source::Triv::int64(100),
                triv2: source::Triv::int64(101),
            })]
            .into_iter()
            .collect(),
            value: Box::new(source::Value::triv(source::Triv::name(
                "x".into(),
            ))),
        }),
    });
    let actual = compile(p).unwrap();
    utils::reset_all_indices();
    let expected = "L.main.0:
\tmov r9, 100
\tadd r9, 101
\tadd rax, r9
\tjmp L.done
L.done:
";
    assert_eq!(actual, expected);
}

// #[test]
// #[serial]
// fn book_example_4() {
//     let program = source::ValuesLang(source::P::module(source::Tail::r#let {
//         bindings: vec![(
//             "x".into(),
//             source::Value::triv(source::Triv::int64(3)),
//         )]
//         .into_iter()
//         .collect(),
//         tail: Box::new(source::Tail::r#let {
//             bindings: vec![(
//                 "x".into(),
//                 source::Value::triv(source::Triv::int64(2)),
//             )]
//             .into_iter()
//             .collect(),
//             tail: Box::new(source::Tail::value(
//                 source::Value::binop_triv_triv {
//                     binop: utils::Binop::plus,
//                     triv1: source::Triv::name("x".into()),
//                     triv2: source::Triv::name("x".into()),
//                 },
//             )),
//         }),
//     }));
//     let paren_x64 = compile(program);
//     let x64 = paren_x64.clone().generate_x64();
//     let result = paren_x64.link_paren_x64().interp_loop();
//     assert_eq!(result, 4);
//     println!("{}", x64);
// }

// #[test]
// #[serial]
// fn book_example_5() {
//     let program = source::ValuesLang(source::P::module(source::Tail::r#let {
//         bindings: vec![(
//             "x".into(),
//             source::Value::triv(source::Triv::int64(3)),
//         )]
//         .into_iter()
//         .collect(),
//         tail: Box::new(source::Tail::r#let {
//             bindings: vec![
//                 ("x".into(), source::Value::triv(source::Triv::int64(2))),
//                 ("y".into(), source::Value::triv(source::Triv::int64(3))),
//                 ("z".into(), source::Value::triv(source::Triv::int64(4))),
//             ]
//             .into_iter()
//             .collect(),
//             tail: Box::new(source::Tail::value(
//                 source::Value::binop_triv_triv {
//                     binop: utils::Binop::plus,
//                     triv1: source::Triv::name("y".into()),
//                     triv2: source::Triv::name("z".into()),
//                 },
//             )),
//         }),
//     }));
//     let paren_x64 = compile(program);
//     let x64 = paren_x64.clone().generate_x64();
//     let result = paren_x64.link_paren_x64().interp_loop();
//     assert_eq!(result, 7);
//     println!("{}", x64);
// }
