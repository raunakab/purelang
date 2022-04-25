use crate::cpsc411;

#[derive(Clone)]
pub enum P {
    begin { ss: Vec<S> },
}

#[derive(Clone)]
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
    jump_trg {
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

pub type Loc = super::target::Loc;

#[derive(Clone)]
pub enum Triv {
    trg { trg: Trg },
    int64 { int64: i64 },
}

pub type Opand = super::target::Opand;

#[derive(Debug, Clone)]
pub enum Trg {
    reg { reg: cpsc411::Reg },
    label { label: cpsc411::Label },
}
