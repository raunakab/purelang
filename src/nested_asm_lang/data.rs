use crate::cpsc411;
use crate::cpsc411::Fvar;
use crate::cpsc411::Reg;

pub struct P {
    pub tail: Tail,
}

pub enum Tail {
    halt {
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
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
    begin {
        effects: Vec<Effect>,
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
