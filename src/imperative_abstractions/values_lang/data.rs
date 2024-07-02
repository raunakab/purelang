use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum P {
    module { lambdas: Vec<Lambda>, tail: Tail },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Lambda {
    pub name: utils::Name,
    pub args: Vec<utils::Name>,
    pub tail: Tail,
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
    r#let {
        bindings: Bindings,
        pred: Box<Self>,
    },
    r#if {
        pred1: Box<Self>,
        pred2: Box<Self>,
        pred3: Box<Self>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Tail {
    value(Value),
    r#let {
        bindings: Bindings,
        tail: Box<Self>,
    },
    r#if {
        pred: Pred,
        tail1: Box<Self>,
        tail2: Box<Self>,
    },
    call {
        name: utils::Name,
        args: Vec<Triv>,
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
    r#let {
        bindings: Bindings,
        value: Box<Self>,
    },
    r#if {
        pred: Pred,
        value1: Box<Self>,
        value2: Box<Self>,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Triv {
    int64(i64),
    name(utils::Name),
}

pub type Bindings = Vec<(utils::Name, Value)>;
