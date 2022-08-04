use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    module(Tail),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Tail {
    value(Value),
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
    r#if {
        pred: Pred,
        tail1: Box<Self>,
        tail2: Box<Self>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Pred {
    relop {
        relop: utils::Relop,
        triv1: Triv,
        triv2: Triv,
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

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Effect {
    set_aloc_value {
        aloc: utils::Aloc,
        value: Value,
    },
    begin(Vec<Effect>),
    r#if {
        pred: Pred,
        effect1: Box<Self>,
        effect2: Box<Self>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Value {
    triv(Triv),
    binop_triv_triv {
        binop: utils::Binop,
        triv1: Triv,
        triv2: Triv,
    },
    begin {
        effects: Vec<Effect>,
        value: Box<Value>,
    },
    r#if {
        pred: Pred,
        value1: Box<Self>,
        value2: Box<Self>,
    },
}

pub type Triv = super::target::Triv;
