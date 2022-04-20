use crate::cpsc411;

pub enum P {
    module { tail: Tail },
}

pub enum Tail {
    value {
        value: Value,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

pub enum Effect {
    set_aloc_value { aloc: cpsc411::Aloc, value: Value },
    begin { effects: Vec<Effect> },
}

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

pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: cpsc411::Aloc },
}
