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
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}

pub struct Fvar {
    pub index: usize,
}

pub trait Check: Sized {
    fn check(self) -> Result<Self, String>;
}

pub trait Interpret {
    type Output;

    fn interpret(self) -> Self::Output;
}
