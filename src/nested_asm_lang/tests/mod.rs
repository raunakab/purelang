use serial_test::serial;

use crate::block_pred_lang as target;
use crate::cpsc411;
use crate::nested_asm_lang as source;

#[test]
#[serial]
fn basic_halt() {
    let program = source::NestedAsmLang(source::P::module(source::Tail::halt(
        source::Triv::int64(5),
    )));

    let actual = program.expose_basic_blocks();

    cpsc411::reset_all_indices();

    let expected =
        target::BlockPredLang(target::P::module(vec![target::B::define {
            label: cpsc411::Label::new_with_name("main"),
            tail: target::Tail::halt(target::Opand::int64(5)),
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn with_no_effects() {
    let program =
        source::NestedAsmLang(source::P::module(source::Tail::begin {
            effects: vec![],
            tail: Box::new(source::Tail::halt(source::Triv::int64(5))),
        }));

    let actual = program.expose_basic_blocks();

    cpsc411::reset_all_indices();

    let expected =
        target::BlockPredLang(target::P::module(vec![target::B::define {
            label: cpsc411::Label::new_with_name("main"),
            tail: target::Tail::halt(target::Opand::int64(5)),
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn with_effect() {
    let program =
        source::NestedAsmLang(source::P::module(source::Tail::begin {
            effects: vec![source::Effect::set {
                loc: source::Loc::reg(cpsc411::Reg::r10),
                triv: source::Triv::int64(5),
            }],
            tail: Box::new(source::Tail::halt(source::Triv::int64(5))),
        }));

    let actual = program.expose_basic_blocks();

    cpsc411::reset_all_indices();

    let expected =
        target::BlockPredLang(target::P::module(vec![target::B::define {
            label: cpsc411::Label::new_with_name("main"),
            tail: target::Tail::begin {
                effects: vec![target::Effect::set {
                    loc: target::Loc::reg(cpsc411::Reg::r10),
                    triv: target::Triv::opand(target::Opand::int64(5)),
                }],
                tail: Box::new(target::Tail::halt(target::Opand::int64(5))),
            },
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn nested_begin_with_no_effects() {
    let program =
        source::NestedAsmLang(source::P::module(source::Tail::begin {
            effects: vec![
                source::Effect::set {
                    loc: source::Loc::reg(cpsc411::Reg::r10),
                    triv: source::Triv::int64(5),
                },
                source::Effect::begin(vec![]),
            ],
            tail: Box::new(source::Tail::halt(source::Triv::int64(5))),
        }));

    let actual = program.expose_basic_blocks();

    cpsc411::reset_all_indices();

    let expected =
        target::BlockPredLang(target::P::module(vec![target::B::define {
            label: cpsc411::Label::new_with_name("main"),
            tail: target::Tail::begin {
                effects: vec![target::Effect::set {
                    loc: target::Loc::reg(cpsc411::Reg::r10),
                    triv: target::Triv::opand(target::Opand::int64(5)),
                }],
                tail: Box::new(target::Tail::halt(target::Opand::int64(5))),
            },
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn nested_begin() {
    let program =
        source::NestedAsmLang(source::P::module(source::Tail::begin {
            effects: vec![
                source::Effect::set {
                    loc: source::Loc::reg(cpsc411::Reg::r10),
                    triv: source::Triv::int64(5),
                },
                source::Effect::begin(vec![
                    source::Effect::set {
                        loc: source::Loc::reg(cpsc411::Reg::r11),
                        triv: source::Triv::int64(5),
                    },
                    source::Effect::set {
                        loc: source::Loc::reg(cpsc411::Reg::r12),
                        triv: source::Triv::int64(5),
                    },
                ]),
            ],
            tail: Box::new(source::Tail::halt(source::Triv::int64(5))),
        }));

    let actual = program.expose_basic_blocks();

    cpsc411::reset_all_indices();

    let expected =
        target::BlockPredLang(target::P::module(vec![target::B::define {
            label: cpsc411::Label::new_with_name("main"),
            tail: target::Tail::begin {
                effects: vec![
                    target::Effect::set {
                        loc: target::Loc::reg(cpsc411::Reg::r10),
                        triv: target::Triv::opand(target::Opand::int64(5)),
                    },
                    target::Effect::set {
                        loc: target::Loc::reg(cpsc411::Reg::r11),
                        triv: target::Triv::opand(target::Opand::int64(5)),
                    },
                    target::Effect::set {
                        loc: target::Loc::reg(cpsc411::Reg::r12),
                        triv: target::Triv::opand(target::Opand::int64(5)),
                    },
                ],
                tail: Box::new(target::Tail::halt(target::Opand::int64(5))),
            },
        }]));

    assert_eq!(actual, expected);
}
