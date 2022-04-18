use crate::cpsc411::Binop;
use crate::cpsc411::Reg;
use crate::paren_x64::data::Triv;
use crate::paren_x64::data::P;
use crate::paren_x64::data::S;
use crate::paren_x64::ParenX64;

#[test]
fn interp_basic() {
    let program = ParenX64 {
        p: P::begin {
            ss: vec![S::set_reg_triv {
                reg: Reg::rax,
                triv: Triv::int64 { int64: 0 },
            }],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
fn interp_intermediary() {
    let program = ParenX64 {
        p: P::begin {
            ss: vec![
                S::set_reg_triv {
                    reg: Reg::rbx,
                    triv: Triv::int64 { int64: 12 },
                },
                S::set_reg_triv {
                    reg: Reg::rax,
                    triv: Triv::int64 { int64: 0 },
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
fn interp_by_setting_with_another_register() {
    let program = ParenX64 {
        p: P::begin {
            ss: vec![
                S::set_reg_triv {
                    reg: Reg::rbx,
                    triv: Triv::int64 { int64: 12 },
                },
                S::set_reg_triv {
                    reg: Reg::rax,
                    triv: Triv::reg { reg: Reg::rbx },
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 12);
}

#[test]
fn interp_with_addition() {
    let program = ParenX64 {
        p: P::begin {
            ss: vec![
                S::set_reg_triv {
                    reg: Reg::rax,
                    triv: Triv::int64 { int64: 12 },
                },
                S::set_reg_binop_reg_int32 {
                    reg: Reg::rax,
                    binop: Binop::plus,
                    int32: 1,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 13);
}

#[test]
fn interp_with_multiplication() {
    let program = ParenX64 {
        p: P::begin {
            ss: vec![
                S::set_reg_triv {
                    reg: Reg::rax,
                    triv: Triv::int64 { int64: 12 },
                },
                S::set_reg_binop_reg_int32 {
                    reg: Reg::rax,
                    binop: Binop::multiply,
                    int32: 2,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 24);
}
