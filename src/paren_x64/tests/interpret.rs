use serial_test::serial;

use crate::cpsc411;
use crate::cpsc411::Interpret;
use crate::paren_x64 as source;

#[test]
#[serial]
fn interp_basic() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![source::S::set_reg_triv {
                reg: cpsc411::Reg::rax,
                triv: source::Triv::int64 { int64: 0 },
            }],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
#[serial]
fn interp_intermediary() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rbx,
                    triv: source::Triv::int64 { int64: 12 },
                },
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 0 },
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
#[serial]
fn interp_by_setting_with_another_register() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rbx,
                    triv: source::Triv::int64 { int64: 12 },
                },
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::trg {
                        trg: source::Trg::reg {
                            reg: cpsc411::Reg::rbx,
                        },
                    },
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 12);
}

#[test]
#[serial]
fn interp_with_addition() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 12 },
                },
                source::S::set_reg_binop_reg_int32 {
                    reg: cpsc411::Reg::rax,
                    binop: cpsc411::Binop::plus,
                    int32: 1,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 13);
}

#[test]
#[serial]
fn interp_with_multiplication() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 12 },
                },
                source::S::set_reg_binop_reg_int32 {
                    reg: cpsc411::Reg::rax,
                    binop: cpsc411::Binop::multiply,
                    int32: 2,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 24);
}

#[test]
#[serial]
fn interp_with_label() {
    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 20 },
                },
                source::S::jump_trg {
                    trg: source::Trg::label {
                        label: cpsc411::Label::halt_label(),
                    },
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 20);
}

#[test]
#[serial]
fn interp_with_multiple_labeled_jumps() {
    let label = cpsc411::Label::new_with_name("start");
    let label2 = cpsc411::Label::new_with_name("start2");

    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 90 },
                },
                source::S::jump_trg {
                    trg: source::Trg::label {
                        label: cpsc411::Label::halt_label(),
                    },
                },
                source::S::with_label {
                    label,
                    s: Box::new(source::S::with_label {
                        label: label2,
                        s: Box::new(source::S::set_reg_triv {
                            reg: cpsc411::Reg::rax,
                            triv: source::Triv::int64 { int64: 100 },
                        }),
                    }),
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 90);
}

#[test]
#[serial]
fn interp_with_jumping_back_and_forth() {
    let label = cpsc411::Label::new_with_name("blah");
    let jumper_label = cpsc411::Label::new_with_name("jumper");

    let program = source::ParenX64 {
        p: source::P::begin {
            ss: vec![
                source::S::jump_trg {
                    trg: source::Trg::label {
                        label: jumper_label.clone(),
                    },
                },
                source::S::with_label {
                    label: label.clone(),
                    s: Box::new(source::S::set_reg_triv {
                        reg: cpsc411::Reg::rax,
                        triv: source::Triv::int64 { int64: 80 },
                    }),
                },
                source::S::set_reg_triv {
                    reg: cpsc411::Reg::rax,
                    triv: source::Triv::int64 { int64: 70 },
                },
                source::S::jump_trg {
                    trg: source::Trg::label {
                        label: cpsc411::Label::halt_label(),
                    },
                },
                source::S::with_label {
                    label: jumper_label.clone(),
                    s: Box::new(source::S::jump_trg {
                        trg: source::Trg::label {
                            label: label.clone(),
                        },
                    }),
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 70);
}
