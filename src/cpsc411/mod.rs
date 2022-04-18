use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref FVAR_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Aloc {
    pub name: String,
    pub index: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Reg {
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

impl Reg {
    pub fn current_frame_base_pointer() -> Self {
        Self::rbp
    }

    pub fn current_auxiliary_registers() -> (Self, Self) {
        (Self::r10, Self::r11)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fvar {
    pub index: usize,
}

impl Fvar {
    pub fn fresh() -> Self {
        let mut fvar_index = FVAR_INDEX.lock().unwrap();
        let index = *fvar_index;
        *fvar_index += 1;

        Self { index }
    }
}

#[derive(Default)]
pub struct Info<Loc> {
    pub locals: HashSet<Aloc>,
    pub assignment: HashMap<Aloc, Loc>,
}

pub trait Check: Sized {
    fn check(self) -> Result<Self, String>;
}

pub trait Interpret {
    type Output;

    fn interpret(self) -> Self::Output;
}
