use std::hash::Hash;

use crate::cpsc411;

#[derive(Clone)]
pub enum P {
    begin { ss: Vec<S> },
}

#[derive(Clone)]
pub enum S {
    set_addr_int32 {
        addr: Addr,
        int32: i32,
    },
    set_addr_trg {
        addr: Addr,
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
    jump_trg {
        trg: Trg,
    },
    compare_reg_opand_jump_if {
        reg: cpsc411::Reg,
        opand: Opand,
        relop: cpsc411::Relop,
        pc_addr: cpsc411::PcAddr,
    },
    nop,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Loc {
    reg { reg: cpsc411::Reg },
    addr { addr: Addr },
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Addr {
    pub fbp: cpsc411::Reg,
    pub disp_offset: usize,
}

#[derive(Clone)]
pub enum Triv {
    trg { trg: Trg },
    int64 { int64: i64 },
}

#[derive(Clone)]
pub enum Opand {
    int64 { int64: i64 },
    reg { reg: cpsc411::Reg },
}

#[derive(Debug, Clone)]
pub enum Trg {
    reg { reg: cpsc411::Reg },
    pc_addr { pc_addr: cpsc411::PcAddr },
}
