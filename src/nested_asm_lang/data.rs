use crate::cpsc411;

pub enum P {
    module { tail: Tail },
}

pub enum Pred {
    relop {
        loc: Loc,
        triv: Triv,
    },
    r#true,
    r#false,
    not {
        pred: Box<Self>,
    },
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
    halt {
        triv: Triv,
    },
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
        binop: cpsc411::Binop,
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
    },
    r#if {
        pred: Pred,
        effect1: Box<Self>,
        effect2: Box<Self>,
    },
}

pub type Loc = super::target::Loc;

pub type Triv = super::target::Opand;
