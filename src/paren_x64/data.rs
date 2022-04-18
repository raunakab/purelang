use std::hash::Hash;

use crate::cpsc411::Binop;
use crate::cpsc411::Reg;

pub enum P {
    begin { ss: Vec<S> },
}

pub enum S {
    set_addr_int32 { addr: Addr, int32: i32 },
    set_addr_reg { addr: Addr, reg: Reg },
    set_reg_loc { reg: Reg, loc: Loc },
    set_reg_triv { reg: Reg, triv: Triv },
    set_reg_binop_reg_int32 { reg: Reg, binop: Binop, int32: i32 },
    set_reg_binop_reg_loc { reg: Reg, binop: Binop, loc: Loc },
}

pub enum Loc {
    reg { reg: Reg },
    addr { addr: Addr },
}

#[derive(Hash, PartialEq, Eq)]
pub struct Addr {
    pub fbp: Reg,
    pub disp_offset: usize,
}

pub enum Triv {
    reg { reg: Reg },
    int64 { int64: i64 },
}
