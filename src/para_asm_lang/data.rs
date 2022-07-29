use crate::cpsc411;

pub enum P {
    begin(Vec<S>),
}

pub enum S {
    halt(Opand),
    set_loc_triv {
        loc: Loc,
        triv: Triv,
    },
    set_loc_binop_opand {
        loc: Loc,
        binop: cpsc411::Binop,
        opand: Opand,
    },
    jump(Trg),
    with_label {
        label: cpsc411::Label,
        s: Box<Self>,
    },
    compare_jump {
        loc: Loc,
        opand: Opand,
        relop: cpsc411::Relop,
        trg: Trg,
    },
    nop,
}

pub type Loc = super::target::Loc;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Triv {
    opand(Opand),
    label(cpsc411::Label),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Trg {
    label(cpsc411::Label),
    loc(Loc),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Opand {
    int64(i64),
    loc(Loc),
}
