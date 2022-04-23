use serial_test::serial;

use crate::asm_lang as source;
use crate::cpsc411;
use crate::nested_asm_lang as target;

#[test]
#[serial]
#[ignore = "Registers get randomly assigned..."]
fn basic() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info {
                locals: Some(vec![aloc.clone()].into_iter().collect()),
                conflicts: Some(cpsc411::Graph::new_with_graph(&[(
                    aloc.clone(),
                    &[],
                )])),
                ..Default::default()
            },
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64 { int64: 42 },
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.assign_registers();

    match p {
        source::P::module {
            info: cpsc411::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::reg {
                    reg: cpsc411::Reg::r9
                }),]
                .into_iter()
                .collect(),
            );
        },
    }
}

#[test]
#[serial]
fn basic_without_registers() {
    cpsc411::Reg::set_current_assignable_registers(
        vec![].into_iter().collect(),
    );

    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info {
                locals: Some(vec![aloc.clone()].into_iter().collect()),
                conflicts: Some(cpsc411::Graph::new_with_graph(&[(
                    aloc.clone(),
                    &[],
                )])),
                ..Default::default()
            },
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64 { int64: 42 },
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.assign_registers();

    cpsc411::reset_all_indices();

    match p {
        source::P::module {
            info: cpsc411::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::fvar {
                    fvar: cpsc411::Fvar::fresh()
                }),]
                .into_iter()
                .collect(),
            );
        },
    };

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
#[ignore = "Registers get randomly assigned..."]
fn intermediary() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info {
                locals: Some(vec![aloc.clone()].into_iter().collect()),
                conflicts: Some(cpsc411::Graph::new_with_graph(&[(
                    aloc.clone(),
                    &[],
                )])),
                ..Default::default()
            },
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64 { int64: 42 },
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.assign_registers();

    match p {
        source::P::module {
            info: cpsc411::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::reg {
                    reg: cpsc411::Reg::r9
                }),]
                .into_iter()
                .collect(),
            );
        },
    }
}
