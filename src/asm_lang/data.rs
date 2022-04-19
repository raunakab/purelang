use crate::cpsc411;

#[derive(Debug, PartialEq, Eq)]
pub enum P {
    module {
        info: cpsc411::Info<super::target::Loc>,
        tail: Tail,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tail {
    halt {
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
