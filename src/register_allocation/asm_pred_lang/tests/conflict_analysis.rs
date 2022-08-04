use std::collections::HashSet;

use serial_test::serial;

use crate::register_allocation::asm_pred_lang as source;
use crate::utils;

#[test]
#[serial]
fn basic() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info {
            locals: Some(vec![aloc.clone()].into_iter().collect()),
            undead_out: Some(utils::Node::tree {
                tree: utils::Tree {
                    nodes: vec![
                        utils::Node::alocs {
                            alocs: vec![aloc.clone()].into_iter().collect(),
                        },
                        utils::Node::alocs {
                            alocs: HashSet::default(),
                        },
                    ],
                },
            }),
            ..Default::default()
        },
        tail: source::Tail::begin {
            effects: vec![],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmPredLang(p) = program.conflict_analysis();

    match p {
        source::P::module {
            info: utils::Info { conflicts, .. },
            ..
        } => {
            assert_eq!(
                conflicts.unwrap(),
                utils::Graph::new_with_graph(&[(aloc, &[]),])
            );
        },
    }
}
