use std::hash::Hash;

use crate::cpsc411;

#[derive(Clone)]
pub enum P {
    begin(Vec<S>),
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
    jump_trg(Trg),
    compare_reg_opand_jump_if {
        reg: cpsc411::Reg,
        opand: Opand,
        relop: cpsc411::Relop,
        pc_addr: cpsc411::PcAddr,
    },
    nop,
}

#[derive(Clone, Hash, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Loc {
    reg(cpsc411::Reg),
    addr(cpsc411::Addr),
}

#[derive(Clone)]
pub enum Triv {
    trg(Trg),
    int64(i64),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Opand {
    int64(i64),
    reg(cpsc411::Reg),
}

#[derive(Debug, Clone)]
pub enum Trg {
    reg(cpsc411::Reg),
    pc_addr(cpsc411::PcAddr),
}
