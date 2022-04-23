use std::collections::HashSet;

use serial_test::serial;

use crate::asm_lang as source;
use crate::cpsc411;

#[test]
#[serial]
fn basic_recursion_depth_0() {
    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::halt {
                triv: source::Triv::int64 { int64: 10 },
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();

    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), cpsc411::Node::alocs {
            alocs: HashSet::default()
        },),
    }
}

#[test]
#[serial]
fn basic_recursion_depth_1() {
    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::int64 { int64: 14 },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
            tree: cpsc411::Tree {
                nodes: vec![cpsc411::Node::alocs {
                    alocs: HashSet::default()
                }]
            }
        }),
    }
}

#[test]
#[serial]
fn basic_recursion_depth_2() {
    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![],
                tail: Box::new(source::Tail::begin {
                    effects: vec![],
                    tail: Box::new(source::Tail::halt {
                        triv: source::Triv::int64 { int64: 14 },
                    }),
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
            tree: cpsc411::Tree {
                nodes: vec![cpsc411::Node::tree {
                    tree: cpsc411::Tree {
                        nodes: vec![cpsc411::Node::alocs {
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
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![source::Effect::set_aloc_triv {
                    aloc: aloc.clone(),
                    triv: source::Triv::int64 { int64: 5 },
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
            tree: cpsc411::Tree {
                nodes: vec![
                    cpsc411::Node::alocs {
                        alocs: vec![aloc].into_iter().collect()
                    },
                    cpsc411::Node::alocs {
                        alocs: HashSet::default()
                    },
                ]
            }
        }),
    }

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_begins_with_multiple_effects() {
    let aloc = cpsc411::Aloc::fresh();
    let aloc2 = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_triv {
                        aloc: aloc2.clone(),
                        triv: source::Triv::int64 { int64: 5 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: source::Triv::int64 { int64: 3 },
                    },
                ],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc {
                        aloc: aloc2.clone(),
                    },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
                tree: cpsc411::Tree {
                    nodes: vec![
                        cpsc411::Node::alocs {
                            alocs: vec![aloc2.clone()].into_iter().collect()
                        },
                        cpsc411::Node::alocs {
                            alocs: vec![aloc2].into_iter().collect()
                        },
                        cpsc411::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    }

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn basic_begins_with_multiple_effects_2() {
    let aloc = cpsc411::Aloc::fresh();
    let aloc2 = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![
                    source::Effect::set_aloc_triv {
                        aloc: aloc2.clone(),
                        triv: source::Triv::int64 { int64: 5 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: source::Triv::int64 { int64: 3 },
                    },
                    source::Effect::set_aloc_triv {
                        aloc: aloc2.clone(),
                        triv: source::Triv::aloc { aloc: aloc.clone() },
                    },
                ],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc {
                        aloc: aloc2.clone(),
                    },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
                tree: cpsc411::Tree {
                    nodes: vec![
                        cpsc411::Node::alocs {
                            alocs: HashSet::default(),
                        },
                        cpsc411::Node::alocs {
                            alocs: vec![aloc.clone()].into_iter().collect(),
                        },
                        cpsc411::Node::alocs {
                            alocs: vec![aloc2].into_iter().collect(),
                        },
                        cpsc411::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    }

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_begins_with_empty_nested_begin() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![source::Effect::begin { effects: vec![] }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
                tree: cpsc411::Tree {
                    nodes: vec![
                        cpsc411::Node::tree {
                            tree: cpsc411::Tree { nodes: vec![] }
                        },
                        cpsc411::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    };

    cpsc411::reset_all_indices();
}

#[test]
#[serial]
fn intermediary_begins_with_nested_begin() {
    let aloc = cpsc411::Aloc::fresh();

    let program = source::AsmLang {
        p: source::P::module {
            info: cpsc411::Info::default(),
            tail: source::Tail::begin {
                effects: vec![source::Effect::begin {
                    effects: vec![source::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: source::Triv::int64 { int64: 19 },
                    }],
                }],
                tail: Box::new(source::Tail::halt {
                    triv: source::Triv::aloc { aloc: aloc.clone() },
                }),
            },
        },
    };

    let source::AsmLang { p } = program.undead_analysis();
    match p {
        source::P::module {
            info: cpsc411::Info { undead_out, .. },
            ..
        } => {
            assert_eq!(undead_out.unwrap(), cpsc411::Node::tree {
                tree: cpsc411::Tree {
                    nodes: vec![
                        cpsc411::Node::tree {
                            tree: cpsc411::Tree {
                                nodes: vec![cpsc411::Node::alocs {
                                    alocs: vec![aloc].into_iter().collect()
                                }],
                            }
                        },
                        cpsc411::Node::alocs {
                            alocs: HashSet::default()
                        },
                    ],
                }
            })
        },
    };

    cpsc411::reset_all_indices();
}
