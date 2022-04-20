use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use derivative::*;
use lazy_static::lazy_static;

use crate::paren_x64;

lazy_static! {
    static ref FVAR_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref ALOC_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

fn fresh_index(asbtract_index: &Arc<Mutex<usize>>) -> usize {
    let mut abstract_index = asbtract_index.lock().unwrap();
    let index = *abstract_index;
    *abstract_index += 1;

    index
}

#[cfg(test)]
fn set_index(asbtract_index: &Arc<Mutex<usize>>) {
    let mut abstract_index = asbtract_index.lock().unwrap();
    *abstract_index = 0usize;
}

#[cfg(test)]
pub fn reset_all_indices() {
    set_index(&ALOC_INDEX);
    set_index(&FVAR_INDEX);
}

#[derive(Debug, Derivative, Clone, Hash, PartialEq, Eq)]
#[derivative(PartialOrd, Ord)]
pub struct Aloc {
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub name: String,

    pub index: usize,
}

impl Aloc {
    pub fn fresh() -> Self {
        let default_name = "tmp";
        let index = fresh_index(&ALOC_INDEX);

        Self {
            name: default_name.into(),
            index,
        }
    }

    pub fn fresh_with_name<I>(name: I) -> Self
    where
        I: Into<String>,
    {
        let index = fresh_index(&ALOC_INDEX);

        Self {
            name: name.into(),
            index,
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fvar {
    pub index: usize,
}

impl Fvar {
    pub fn fresh() -> Self {
        let index = fresh_index(&FVAR_INDEX);
        Self { index }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info<Loc> {
    pub locals: HashSet<Aloc>,
    pub assignment: HashMap<Aloc, Loc>,
}

impl<Loc> Default for Info<Loc> {
    fn default() -> Self {
        Self {
            locals: HashSet::default(),
            assignment: HashMap::default(),
        }
    }
}

pub trait Check: Sized {
    fn check(self) -> Result<Self, String>;
}

pub trait Interpret {
    type Output;

    fn interpret(self) -> Self::Output;
}

pub trait Compile {
    fn compile(self, opt_level: crate::OptLevels) -> paren_x64::ParenX64;
}
