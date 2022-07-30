use serial_test::serial;

use crate::cpsc411;
use crate::cpsc411::Compile;
use crate::cpsc411::Interpret;
use crate::values_lang as source;

#[test]
#[serial]
fn basic() {
    let ir = source::ValuesLang(source::P::module(source::Tail::value(source::Value::triv(source::Triv::int64(9)))))
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    assert_eq!(
        x64,
        "
mov rax, 9"
            .to_string(),
    );

    assert_eq!(result, 9);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_with_let_bindings() {
    let ir = source::ValuesLang(source::P::module(source::Tail::value(source::Value::r#let {
        bindings: vec![("x".into(), source::Value::triv(source::Triv::int64(100)))]
        .into_iter()
        .collect(),
        value: Box::new(source::Value::triv(source::Triv::name("x".into()))),
    })))
    .compile(crate::OptLevels::O1);

    let x64 = ir.clone().generate_x64();
    let result = ir.interpret();

    println!("\n{}\n", x64);

    assert_eq!(
        x64,
        "
mov QWORD [rbp - 0], 100
mov rax, QWORD [rbp - 0]"
            .to_string(),
    );

    assert_eq!(result, 100);

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn book_example_4() {
    let program = source::ValuesLang(source::P::module(source::Tail::r#let {
        bindings: vec![("x".into(), source::Value::triv(source::Triv::int64(3)))]
        .into_iter()
        .collect(),

        tail: Box::new(source::Tail::r#let {
            bindings: vec![("x".into(), source::Value::triv(source::Triv::int64(2)))]
            .into_iter()
            .collect(),
            tail: Box::new(source::Tail::value(source::Value::binop_triv_triv {
                binop: cpsc411::Binop::plus,
                triv1: source::Triv::name("x".into()),
                triv2: source::Triv::name("x".into()),
            })),
        }),
    }));

    let paren_x64 = program.compile(crate::OptLevels::O3);

    let x64 = paren_x64.clone().generate_x64();
    let result = paren_x64.interpret();

    assert_eq!(result, 4);

    println!("{}", x64);
}

#[test]
#[serial]
fn book_example_5() {
    let program = source::ValuesLang(source::P::module(source::Tail::r#let {
        bindings: vec![("x".into(), source::Value::triv(source::Triv::int64(3)))]
        .into_iter()
        .collect(),

        tail: Box::new(source::Tail::r#let {
            bindings: vec![
                ("x".into(), source::Value::triv(source::Triv::int64(2))),
                ("y".into(), source::Value::triv(source::Triv::int64(3))),
                ("z".into(), source::Value::triv(source::Triv::int64(4))),
            ]
            .into_iter()
            .collect(),
            tail: Box::new(source::Tail::value(source::Value::binop_triv_triv {
                binop: cpsc411::Binop::plus,
                triv1: source::Triv::name("y".into()),
                triv2: source::Triv::name("z".into()),
            })),
        }),
    }));

    let paren_x64 = program.compile(crate::OptLevels::O3);

    let x64 = paren_x64.clone().generate_x64();
    let result = paren_x64.interpret();

    assert_eq!(result, 7);

    println!("{}", x64);
}
