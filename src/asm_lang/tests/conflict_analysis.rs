use std::collections::HashSet;

use serial_test::serial;

use crate::asm_lang as source;
use crate::cpsc411;

#[test]
#[serial]
fn basic() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info {
                locals: Some(vec![aloc.clone()].into_iter().collect()),
                undead_out: Some(cpsc411::Node::tree {
                    tree: cpsc411::Tree {
                        nodes: vec![
                            cpsc411::Node::alocs {
                                alocs: vec![aloc.clone()].into_iter().collect(),
                            },
                            cpsc411::Node::alocs {
                                alocs: HashSet::default(),
                            },
                        ],
                    },
                }),
                ..Default::default()
            },
            tail: source::Tail::begin {
                effects: vec![],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.conflict_analysis();

    match p {
        source::P::module {
            info: cpsc411::Info { conflicts, .. },
            ..
        } => {
            assert_eq!(
                conflicts.unwrap(),
                cpsc411::Graph::new_with_graph(&[(aloc, &[]),])
            );
        },
    }
}
