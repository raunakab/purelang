use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    begin(Vec<S>),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum S {
    set_fvar_int32 {
        fvar: utils::Fvar,
        int32: i32,
    },
    set_fvar_trg {
        fvar: utils::Fvar,
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
    compare {
        reg: utils::Reg,
        opand: Opand,
        relop: utils::Relop,
        label: utils::Label,
    },
    nop,
}

#[derive(Clone, Hash, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Loc {
    reg(utils::Reg),
    fvar(utils::Fvar),
}

pub type Triv = super::target::Triv;

pub type Trg = super::target::Trg;

pub type Opand = super::target::Opand;
