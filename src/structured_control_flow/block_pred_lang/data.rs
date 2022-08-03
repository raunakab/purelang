use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    module(Vec<B>),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum B {
    define { label: utils::Label, tail: Tail },
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Pred {
    relop {
        relop: utils::Relop,
        loc: Loc,
        opand: Opand,
    },
    r#true,
    r#false,
    not(Box<Self>),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Tail {
    halt(Opand),
    jump(Trg),
    begin {
        effects: Vec<Effect>,
        tail: Box<Self>,
    },
    r#if {
        pred: Pred,
        trg1: Trg,
        trg2: Trg,
    },
}

pub type Effect = super::target::Effect;

pub type Triv = super::target::Triv;

pub type Opand = super::target::Opand;

pub type Loc = super::target::Loc;

pub type Trg = super::target::Trg;
