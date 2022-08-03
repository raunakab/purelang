use std::collections::HashMap;

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
pub enum P {
    module(Tail),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pred {
    relop_triv_triv {
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
        pred: Box<Self>,
        csqt: Box<Self>,
        antc: Box<Self>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tail {
    value(Value),
    r#let {
        bindings: HashMap<Name, Value>,
        tail: Box<Self>,
    },
    r#if {
        pred: Pred,
        csqt: Box<Self>,
        antc: Box<Self>,
    },
}

#[derive(Debug, PartialEq, Eq)]
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
        csqt: Box<Self>,
        antc: Box<Self>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64(i64),
    name(Name),
}

pub type Name = String;

pub type Bindings = HashMap<Name, Value>;
