use crate::cpsc411;

pub enum P {
    module { bs: Vec<B> },
}

pub enum B {
    define_label_tail { label: cpsc411::Label, tail: Tail },
}

pub enum Tail {
    halt {
        opand: Opand,
    },
    jump {
        trg: Trg,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
    r#if {
        relop: cpsc411::Relop,
        loc: Loc,
        opand: Opand,
        trg1: Trg,
        trg2: Trg,
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
        opand: Opand,
    },
}

pub type Triv = super::target::Triv;

pub type Opand = super::target::Opand;

pub type Loc = super::target::Loc;

pub type Trg = super::target::Trg;
