use crate::utils;

pub enum P {
    module(Vec<B>),
}

pub enum B {
    define_label_tail { label: utils::Label, tail: Tail },
}

pub enum Tail {
    halt(Opand),
    jump(Trg),
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
    r#if {
        relop: utils::Relop,
        loc: Loc,
        opand: Opand,
        trg1: Trg,
        trg2: Trg,
    },
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Effect {
    set {
        loc: Loc,
        triv: Triv,
    },
    set_binop {
        loc: Loc,
        binop: utils::Binop,
        opand: Opand,
    },
}

pub type Triv = super::target::Triv;

pub type Opand = super::target::Opand;

pub type Loc = super::target::Loc;

pub type Trg = super::target::Trg;
