use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    module { lambdas: Vec<Lambda>, tail: Tail },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Lambda {
    pub label: utils::Label,
    pub tail: Tail,
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
    jump {
        trg: u8,
        locs: Vec<u8>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Pred {
    relop {
        relop: utils::Relop,
        opand1: Opand,
        opand2: Opand,
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
    binop {
        binop: utils::Binop,
        opand1: Opand,
        opand2: Opand,
    },
}

pub type Opand = super::target::Triv;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Triv {
    opand(Opand),
    label(utils::Label),
}
