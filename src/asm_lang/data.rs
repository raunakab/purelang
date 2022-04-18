use crate::cpsc411;

pub enum P {
    module {
        info: cpsc411::Info<super::target::Loc>,
        tail: Tail,
    },
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
    set_aloc_triv {
        aloc: cpsc411::Aloc,
        triv: Triv,
    },
    set_aloc_binop_aloc_triv {
        aloc: cpsc411::Aloc,
        binop: cpsc411::Binop,
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
    },
}

pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
