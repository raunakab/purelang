use crate::cpsc411;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    begin(Vec<S>),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum S {
    set_addr_int32 {
        addr: cpsc411::Addr,
        int32: i32,
    },
    set_addr_trg {
        addr: cpsc411::Addr,
        trg: Trg,
    },
    set_reg_loc {
        reg: cpsc411::Reg,
        loc: Loc,
    },
    set_reg_triv {
        reg: cpsc411::Reg,
        triv: Triv,
    },
    set_reg_binop_reg_int32 {
        reg: cpsc411::Reg,
        binop: cpsc411::Binop,
        int32: i32,
    },
    set_reg_binop_reg_loc {
        reg: cpsc411::Reg,
        binop: cpsc411::Binop,
        loc: Loc,
    },
    with_label {
        label: cpsc411::Label,
        s: Box<Self>,
    },
    jump_trg(Trg),
    compare_reg_opand_jump_if {
        reg: cpsc411::Reg,
        opand: Opand,
        relop: cpsc411::Relop,
        label: cpsc411::Label,
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
    reg(cpsc411::Reg),
    label(cpsc411::Label),
}

pub type Loc = super::target::Loc;

pub type Opand = super::target::Opand;
