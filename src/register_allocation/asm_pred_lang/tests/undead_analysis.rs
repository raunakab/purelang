use std::collections::HashSet;

use serial_test::serial;

use crate::register_allocation::asm_pred_lang as source;
use crate::utils;

#[test]
#[serial]
fn basic_recursion_depth_0() {
    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::halt(source::Triv::int64(10)),
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), utils::Node::alocs {
            alocs: HashSet::default()
        },),
    }
}

#[test]
#[serial]
fn basic_recursion_depth_1() {
    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![],
            tail: Box::new(source::Tail::halt(source::Triv::int64(10))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), utils::Node::tree {
            tree: utils::Tree {
                nodes: vec![utils::Node::alocs {
                    alocs: HashSet::default()
                }]
            }
        }),
    }
}

#[test]
#[serial]
fn basic_recursion_depth_2() {
    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![],
            tail: Box::new(source::Tail::begin {
                effects: vec![],
                tail: Box::new(source::Tail::halt(source::Triv::int64(10))),
            }),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), utils::Node::tree {
            tree: utils::Tree {
                nodes: vec![utils::Node::tree {
                    tree: utils::Tree {
                        nodes: vec![utils::Node::alocs {
                            alocs: HashSet::default()
                        }],
                    }
                },]
            }
        }),
    }
}

#[test]
#[serial]
fn basic_begins_with_triv_ref() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![source::Effect::set_aloc_triv {
                aloc: aloc.clone(),
                triv: source::Triv::int64(10),
            }],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), utils::Node::tree {
            tree: utils::Tree {
                nodes: vec![
                    utils::Node::alocs {
                        alocs: vec![aloc].into_iter().collect()
                    },
                    utils::Node::alocs {
                        alocs: HashSet::default()
                    },
                ]
            }
        }),
    }

    utils::reset_all_indices();
}

#[test]
#[serial]
fn basic_begins_with_multiple_effects() {
    let aloc = utils::Aloc::fresh();
    let aloc2 = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![
                source::Effect::set_aloc_triv {
                    aloc: aloc2.clone(),
                    triv: source::Triv::int64(10),
                },
                source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64(10),
                },
            ],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc2.clone(),
            ))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), utils::Node::tree {
                tree: utils::Tree {
                    nodes: vec![
                        utils::Node::alocs {
                            alocs: vec![aloc2.clone()].into_iter().collect()
                        },
                        utils::Node::alocs {
                            alocs: vec![aloc2].into_iter().collect()
                        },
                        utils::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    }

    utils::reset_all_indices();
}

#[test]
#[serial]
fn basic_begins_with_multiple_effects_2() {
    let aloc = utils::Aloc::fresh();
    let aloc2 = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![
                source::Effect::set_aloc_triv {
                    aloc: aloc2.clone(),
                    triv: source::Triv::int64(10),
                },
                source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64(10),
                },
                source::Effect::set_aloc_triv {
                    aloc: aloc2.clone(),
                    triv: source::Triv::aloc(aloc.clone()),
                },
            ],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc2.clone(),
            ))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), utils::Node::tree {
                tree: utils::Tree {
                    nodes: vec![
                        utils::Node::alocs {
                            alocs: HashSet::default(),
                        },
                        utils::Node::alocs {
                            alocs: vec![aloc.clone()].into_iter().collect(),
                        },
                        utils::Node::alocs {
                            alocs: vec![aloc2].into_iter().collect(),
                        },
                        utils::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    }

    utils::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_begins_with_empty_nested_begin() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![source::Effect::begin(vec![])],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(aloc))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), utils::Node::tree {
                tree: utils::Tree {
                    nodes: vec![
                        utils::Node::tree {
                            tree: utils::Tree { nodes: vec![] }
                        },
                        utils::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    };

    utils::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_begins_with_nested_begin() {
    let aloc = utils::Aloc::fresh();

    let program = source::AsmPredLang(source::P::module {
        info: utils::Info::default(),
        tail: source::Tail::begin {
            effects: vec![source::Effect::begin(vec![
                source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64(10),
                },
            ])],
            tail: Box::new(source::Tail::halt(source::Triv::aloc(
                aloc.clone(),
            ))),
        },
    });

    let source::AsmPredLang(p) = program.undead_analysis();

    match p {
        source::P::module {
            info: utils::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), utils::Node::tree {
                tree: utils::Tree {
                    nodes: vec![
                        utils::Node::tree {
                            tree: utils::Tree {
                                nodes: vec![utils::Node::alocs {
                                    alocs: vec![aloc].into_iter().collect()
                                }],
                            }
                        },
                        utils::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    };

    utils::reset_all_indices();
}
