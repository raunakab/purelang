use crate::cpsc411;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum P {
    module {
        info: cpsc411::Info<super::target::Loc>,
        tail: Tail,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Tail {
    halt {
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
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

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
