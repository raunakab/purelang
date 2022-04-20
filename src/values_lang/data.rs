use std::collections::HashMap;

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
        bindings: HashMap<Name, Value>,
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
        bindings: HashMap<Name, Value>,
        value: Box<Value>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64 { int64: i64 },
    name { name: Name },
}

type Name = String;
