pub struct Aloc {
    name: String,
    index: usize,
}

// /// Can only be multiples of 8.
// /// (i.e., 0, 8, 16, etc.)
// type DispOffset = usize;

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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}

pub trait Check: Sized {
    fn check(self) -> Result<Self, String>;
}

pub trait Interpret {
    type Output;

    fn interpret(self) -> Self::Output;
}
