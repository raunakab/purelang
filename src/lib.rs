pub mod paren_x64 {
    use std::collections::HashMap;
    use std::hash::Hash;

    #[allow(non_camel_case_types)]
    pub struct Paren_x64 {
        p: P,
    }

    impl Paren_x64 {
        pub fn interpret(self) -> i64 {
            let mut registers = HashMap::<Regs, i64>::new();

            let operate = |binop: Binop, int64_1: i64, int64_2: i64| -> i64 {
                match binop {
                    Binop::plus => int64_1 + int64_2,
                    Binop::multiply => int64_1 * int64_2,
                }
            };

            let interpret_s = |s: S| {
                match s {
                    S::set_reg_int64 { reg, int64 } => {
                        registers.insert(reg, int64);
                    },
                    S::set_reg_reg { reg, reg_other } => {
                        let reg_other_value = registers.get(&reg_other).map(i64::clone).unwrap();
                        registers.insert(reg, reg_other_value);
                    },
                    S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                        let prev_reg_value = registers.get(&reg).map(i64::clone).unwrap();
                        let new_reg_value = operate(binop, prev_reg_value, int32 as i64);
                        registers.insert(reg, new_reg_value);
                    },
                    S::set_reg_binop_reg_reg { reg, binop, reg_other } => {
                        let prev_reg_value = registers.get(&reg).map(i64::clone).unwrap();
                        let reg_other_value = registers.get(&reg_other).map(i64::clone).unwrap();
                        let new_reg_value = operate(binop, prev_reg_value, reg_other_value);
                        registers.insert(reg, new_reg_value);
                    },
                }
            };

            let interpret_p = |p: P| {
                match p {
                    P::begin { ss } => ss.into_iter().for_each(interpret_s),
                }
            };

            interpret_p(self.p);

            registers.get(&Regs::rax).map(i64::clone).unwrap()
        }
    }

    #[allow(non_camel_case_types)]
    pub enum P {
        begin {
            ss: Vec<S>,
        },
    }

    #[allow(non_camel_case_types)]
    pub enum S {
        set_reg_int64 {
            reg: Regs,
            int64: i64,
        },
        set_reg_reg {
            reg: Regs,
            reg_other: Regs,
        },
        set_reg_binop_reg_int32 {
            reg: Regs,
            binop: Binop,
            int32: i32,
        },
        set_reg_binop_reg_reg {
            reg: Regs,
            binop: Binop,
            reg_other: Regs,
        },
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    #[allow(non_camel_case_types)]
    pub enum Binop {
        plus,
        multiply,
    }
}
