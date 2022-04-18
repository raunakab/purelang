use crate::cpsc411::Binop;
use crate::cpsc411::Fvar;
use crate::cpsc411::Reg;

pub enum P {
    begin { ss: Vec<S> },
}

pub enum S {
    set_fvar_int32 { fvar: Fvar, int32: i32 },
    set_fvar_reg { fvar: Fvar, reg: Reg },
    set_reg_loc { reg: Reg, loc: Loc },
    set_reg_triv { reg: Reg, triv: Triv },
    set_reg_binop_reg_int32 { reg: Reg, binop: Binop, int32: i32 },
    set_reg_binop_reg_loc { reg: Reg, binop: Binop, loc: Loc },
}

pub enum Loc {
    reg { reg: Reg },
    fvar { fvar: Fvar },
}

pub enum Triv {
    reg { reg: Reg },
    int64 { int64: i64 },
}
