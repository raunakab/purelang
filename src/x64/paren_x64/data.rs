use crate::utils;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    begin(Vec<S>),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum S {
    set_addr_int32 {
        addr: utils::Addr,
        int32: i32,
    },
    set_addr_trg {
        addr: utils::Addr,
        trg: Trg,
    },
    set_reg_loc {
        reg: utils::Reg,
        loc: Loc,
    },
    set_reg_triv {
        reg: utils::Reg,
        triv: Triv,
    },
    set_reg_binop_reg_int32 {
        reg: utils::Reg,
        binop: utils::Binop,
        int32: i32,
    },
    set_reg_binop_reg_loc {
        reg: utils::Reg,
        binop: utils::Binop,
        loc: Loc,
    },
    with_label {
        label: utils::Label,
        s: Box<Self>,
    },
    jump(Trg),
    compare_reg_opand_jump_if {
        reg: utils::Reg,
        opand: Opand,
        relop: utils::Relop,
        label: utils::Label,
    },
    nop,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Triv {
    trg(Trg),
    int64(i64),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Trg {
    reg(utils::Reg),
    label(utils::Label),
}

pub type Loc = super::target::Loc;

pub type Opand = super::target::Opand;
