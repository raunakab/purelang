use std::collections::HashMap;

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
pub enum P {
    module { tail: Tail },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tail {
    value {
        value: Value,
    },
    r#let {
        bindings: HashMap<utils::Aloc, Value>,
        tail: Box<Tail>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    triv {
        triv: Triv,
    },
    binop_triv_triv {
        binop: utils::Binop,
        triv1: Triv,
        triv2: Triv,
    },
    r#let {
        bindings: HashMap<utils::Aloc, Value>,
        value: Box<Value>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: utils::Aloc },
}
