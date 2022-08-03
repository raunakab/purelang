use crate::utils;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum P {
    module {
        info: utils::Info<super::target::Loc>,
        tail: Tail,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Tail {
    halt(Triv),
    begin {
        effects: Vec<Effect>,
        tail: Box<Self>,
    },
    r#if {
        pred: Pred,
        tail1: Box<Self>,
        tail2: Box<Self>,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Pred {
    relop {
        relop: utils::Relop,
        aloc: utils::Aloc,
        triv: Triv,
    },
    r#true,
    r#false,
    not(Box<Self>),
    begin {
        effects: Vec<Effect>,
        pred: Box<Self>,
    },
    r#if {
        pred1: Box<Self>,
        pred2: Box<Self>,
        pred3: Box<Self>,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Effect {
    set_aloc_triv {
        aloc: utils::Aloc,
        triv: Triv,
    },
    set_aloc_binop_aloc_triv {
        aloc: utils::Aloc,
        binop: utils::Binop,
        triv: Triv,
    },
    begin(Vec<Self>),
    r#if {
        pred: Pred,
        effect1: Box<Self>,
        effect2: Box<Self>,
    },
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Triv {
    int64(i64),
    aloc(utils::Aloc),
}
