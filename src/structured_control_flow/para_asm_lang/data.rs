use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    begin(Vec<S>),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum S {
    halt(Opand),
    set_loc_triv {
        loc: Loc,
        triv: Triv,
    },
    set_loc_binop_opand {
        loc: Loc,
        binop: utils::Binop,
        opand: Opand,
    },
    jump(Trg),
    with_label {
        label: utils::Label,
        s: Box<Self>,
    },
    compare_jump {
        loc: Loc,
        opand: Opand,
        relop: utils::Relop,
        trg: Trg,
    },
    nop,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Triv {
    opand(Opand),
    label(utils::Label),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Trg {
    label(utils::Label),
    loc(Loc),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Opand {
    int64(i64),
    loc(Loc),
}

pub type Loc = super::target::Loc;
