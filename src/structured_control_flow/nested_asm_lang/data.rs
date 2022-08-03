use crate::utils;

pub enum P {
    module(Tail),
}

pub enum Pred {
    relop {
        relop: utils::Relop,
        loc: Loc,
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

pub enum Effect {
    set {
        loc: Loc,
        triv: Triv,
    },
    set_binop {
        loc: Loc,
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

pub type Loc = super::target::Loc;

pub type Triv = super::target::Opand;
