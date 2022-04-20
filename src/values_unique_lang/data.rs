use std::collections::HashMap;

use crate::cpsc411;

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
        bindings: HashMap<cpsc411::Aloc, Value>,
        tail: Box<Tail>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    triv {
        triv: Triv,
    },
    binop_triv_triv {
        triv1: Triv,
        triv2: Triv,
    },
    r#let {
        bindings: HashMap<cpsc411::Aloc, Value>,
        value: Box<Value>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
