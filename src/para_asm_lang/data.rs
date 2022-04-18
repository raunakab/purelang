use crate::cpsc411;
use crate::cpsc411::Fvar;
use crate::cpsc411::Reg;

pub enum P {
    begin { effects: Vec<Effect>, halt: Halt },
}

pub enum Effect {
    set_loc_triv {
        loc: Loc,
        triv: Triv,
    },
    set_loc_binop_triv {
        loc: Loc,
        binop: cpsc411::Binop,
        triv: Triv,
    },
}

pub enum Loc {
    reg { reg: Reg },
    fvar { fvar: Fvar },
}

pub enum Triv {
    loc { loc: Loc },
    int64 { int64: i64 },
}

pub struct Halt {
    pub triv: Triv,
}
