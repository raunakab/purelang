use serial_test::serial;

use crate::register_allocation::asm_pred_lang as source;
use crate::utils;
use crate::structured_control_flow::nested_asm_lang as target;

#[test]
#[serial]
#[ignore = "Registers get randomly assigned..."]
fn basic() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmLang(source::P::module {
        info: utils::Info {
            locals: Some(vec![aloc.clone()].into_iter().collect()),
            conflicts: Some(utils::Graph::new_with_graph(&[(
                aloc.clone(),
                &[],
            )])),
            ..Default::default()
        },
        tail: source::Tail::begin {
            effects: vec![source::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: source::Triv::int64(42),
            }],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmLang(p) = program.assign_registers();

    match p {
        source::P::module {
            info: utils::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::reg(utils::Reg::r9)),]
                    .into_iter()
                    .collect(),
            );
        },
    }
}

#[test]
#[serial]
fn basic_without_registers() {
    utils::Reg::set_current_assignable_registers(
        vec![].into_iter().collect(),
    );

    let aloc = utils::Aloc::fresh();

    let program = source::AsmLang(source::P::module {
        info: utils::Info {
            locals: Some(vec![aloc.clone()].into_iter().collect()),
            conflicts: Some(utils::Graph::new_with_graph(&[(
                aloc.clone(),
                &[],
            )])),
            ..Default::default()
        },
        tail: source::Tail::begin {
            effects: vec![source::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: source::Triv::int64(42),
            }],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmLang(p) = program.assign_registers();

    utils::reset_all_indices();

    match p {
        source::P::module {
            info: utils::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::fvar(utils::Fvar::fresh())),]
                    .into_iter()
                    .collect(),
            );
        },
    };

    utils::reset_all_indices();
}

#[test]
#[serial]
#[ignore = "Registers get randomly assigned..."]
fn intermediary() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmLang(source::P::module {
        info: utils::Info {
            locals: Some(vec![aloc.clone()].into_iter().collect()),
            conflicts: Some(utils::Graph::new_with_graph(&[(
                aloc.clone(),
                &[],
            )])),
            ..Default::default()
        },
        tail: source::Tail::begin {
            effects: vec![source::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: source::Triv::int64(42),
            }],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmLang(p) = program.assign_registers();

    match p {
        source::P::module {
            info: utils::Info { assignment, .. },
            ..
        } => {
            assert_eq!(
                assignment.unwrap(),
                vec![(aloc, target::Loc::reg(utils::Reg::r9)),]
                    .into_iter()
                    .collect(),
            );
    },
    }
}
