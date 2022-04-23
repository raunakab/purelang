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
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    set_aloc_value { aloc: cpsc411::Aloc, value: Value },
    begin { effects: Vec<Effect> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    triv {
        triv: Triv,
    },
    binop_triv_triv {
        binop: cpsc411::Binop,
        triv1: Triv,
        triv2: Triv,
    },
    begin {
        effects: Vec<Effect>,
        value: Box<Value>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
