use serial_test::serial;

use crate::cpsc411;
use crate::cpsc411::Compile;
use crate::cpsc411::Interpret;
use crate::values_lang as source;

#[test]
#[serial]
fn basic() {
    let ir = source::ValuesLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::triv {
                    triv: source::Triv::int64 { int64: 9 },
                },
            },
        },
    }
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
    let ir = source::ValuesLang {
        p: source::P::module {
            tail: source::Tail::value {
                value: source::Value::r#let {
                    bindings: vec![("x".into(), source::Value::triv {
                        triv: source::Triv::int64 { int64: 100 },
                    })]
                    .into_iter()
                    .collect(),
                    value: Box::new(source::Value::triv {
                        triv: source::Triv::name { name: "x".into() },
                    }),
                },
            },
        },
    }
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
