use crate::cpsc411;

pub enum P {
    tail { tail: Tail },
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

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Loc {
    reg { reg: cpsc411::Reg },
    fvar { fvar: cpsc411::Fvar },
}

pub enum Triv {
    int64 { int64: i64 },
    loc { loc: Loc },
}
