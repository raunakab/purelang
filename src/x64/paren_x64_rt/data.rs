use std::hash::Hash;

use crate::utils;

#[derive(Clone)]
pub enum P {
    begin(Vec<S>),
}

#[derive(Clone)]
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
    jump_trg(Trg),
    compare_reg_opand_jump_if {
        reg: utils::Reg,
        opand: Opand,
        relop: utils::Relop,
        pc_addr: utils::PcAddr,
    },
    nop,
}

#[derive(Clone, Hash, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Loc {
    reg(utils::Reg),
    addr(utils::Addr),
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
    reg(utils::Reg),
}

#[derive(Debug, Clone)]
pub enum Trg {
    reg(utils::Reg),
    pc_addr(utils::PcAddr),
}
