use crate::paren_x64::Binop;
use crate::paren_x64::Paren_x64;
use crate::paren_x64::Regs;
use crate::paren_x64::P;
use crate::paren_x64::S;

#[test]
fn interp_basic() {
    let program = Paren_x64 {
        p: P::begin {
            ss: vec![S::set_reg_int64 {
                reg: Regs::rax,
                int64: 0,
            }],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
fn interp_intermediary() {
    let program = Paren_x64 {
        p: P::begin {
            ss: vec![
                S::set_reg_int64 {
                    reg: Regs::rbx,
                    int64: 12,
                },
                S::set_reg_int64 {
                    reg: Regs::rax,
                    int64: 0,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 0);
}

#[test]
fn interp_by_setting_with_another_register() {
    let program = Paren_x64 {
        p: P::begin {
            ss: vec![
                S::set_reg_int64 {
                    reg: Regs::rbx,
                    int64: 12,
                },
                S::set_reg_reg {
                    reg: Regs::rax,
                    reg_other: Regs::rbx,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 12);
}

#[test]
fn interp_with_addition() {
    let program = Paren_x64 {
        p: P::begin {
            ss: vec![
                S::set_reg_int64 {
                    reg: Regs::rax,
                    int64: 12,
                },
                S::set_reg_binop_reg_int32 {
                    reg: Regs::rax,
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
    let program = Paren_x64 {
        p: P::begin {
            ss: vec![
                S::set_reg_int64 {
                    reg: Regs::rax,
                    int64: 12,
                },
                S::set_reg_binop_reg_int32 {
                    reg: Regs::rax,
                    binop: Binop::multiply,
                    int32: 2,
                },
            ],
        },
    };

    let result = program.interpret();

    assert_eq!(result, 24);
}
