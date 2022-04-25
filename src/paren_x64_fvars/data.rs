use crate::cpsc411;

pub enum P {
    begin { ss: Vec<S> },
}

pub enum S {
    set_fvar_int32 {
        fvar: cpsc411::Fvar,
        int32: i32,
    },
    set_fvar_trg {
        fvar: cpsc411::Fvar,
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
    jump {
        trg: Trg,
    },
    compare_reg_opand_jump_if {
        reg: cpsc411::Reg,
        opand: Opand,
        relop: cpsc411::Relop,
        label: cpsc411::Label,
    },
    nop,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Loc {
    reg { reg: cpsc411::Reg },
    fvar { fvar: cpsc411::Fvar },
}

pub type Triv = super::target::Triv;

pub type Trg = super::target::Trg;

pub type Opand = super::target::Opand;
