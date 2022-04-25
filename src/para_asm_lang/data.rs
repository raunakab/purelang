use crate::cpsc411;

pub enum P {
    begin { ss: Vec<S> },
}

pub enum S {
    halt {
        opand: Opand,
    },
    set_loc_triv {
        loc: Loc,
        triv: Triv,
    },
    set_loc_binop_opand {
        loc: Loc,
        binop: cpsc411::Binop,
        opand: Opand,
    },
    jump {
        trg: Trg,
    },
    with_label {
        label: cpsc411::Label,
        s: Box<Self>,
    },
    compare_jump {
        loc: Loc,
        opand: Opand,
        relop: cpsc411::Relop,
        trg: Trg,
    },
    nop,
}

pub type Loc = super::target::Loc;

pub enum Triv {
    opand { opand: Opand },
    label { label: cpsc411::Label },
}

pub enum Trg {
    label { label: cpsc411::Label },
    loc { loc: Loc },
}

pub enum Opand {
    int64 { int64: i64 },
    loc { loc: Loc },
}
