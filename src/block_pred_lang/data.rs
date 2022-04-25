use crate::cpsc411;

pub enum P {
    module { bs: Vec<B> },
}

pub enum B {
    define_label_tail { label: cpsc411::Label, tail: Tail },
}

pub enum Pred {
    relop_loc_opand {
        relop: cpsc411::Relop,
        loc: Loc,
        opand: Opand,
    },
    r#true,
    r#false,
    not {
        pred: Box<Pred>,
    },
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
        pred: Pred,
        trg1: Trg,
        trg2: Trg,
    },
}

pub type Effect = super::target::Effect;

pub type Opand = super::target::Opand;

pub type Loc = super::target::Loc;

pub type Trg = super::target::Trg;
