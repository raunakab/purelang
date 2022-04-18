#[cfg(test)]
mod tests;

use std::hash::Hash;

use crate::cpsc411::Aloc;

pub struct AsmLang {
    p: P,
}

pub enum P {
    module { info: (), tail: Tail },
}

pub enum Tail {
    halt {
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

pub enum Effect {
    set_aloc_triv {
        aloc: Aloc,
        triv: Triv,
    },
    set_aloc_binop_aloc_triv {
        aloc: Aloc,
        binop: Binop,
        triv: Triv,
    },
    begin {
        effects: Vec<Effect>,
        tail: Box<Tail>,
    },
}

pub enum Triv {
    int64 { int64: i64 },
    aloc { aloc: Aloc },
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Regs {
    rsp,
    rbp,
    rax,
    rbx,
    rcx,
    rdx,
    rsi,
    rdi,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}
