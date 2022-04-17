use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[allow(non_camel_case_types)]
pub struct Paren_x64 {
    p: P,
}

impl Paren_x64 {
    pub fn interpret(self) -> i64 {
        let mut registers = HashMap::<Regs, i64>::new();

        fn operate(binop: Binop, int64_1: i64, int64_2: i64) -> i64 {
            match binop {
                Binop::plus => int64_1 + int64_2,
                Binop::multiply => int64_1 * int64_2,
            }
        }

        fn interpret_s(s: S, registers: &mut HashMap<Regs, i64>) {
            match s {
                S::set_reg_int64 { reg, int64 } => {
                    registers.insert(reg, int64);
                },
                S::set_reg_reg { reg, reg_other } => {
                    let reg_other_value =
                        registers.get(&reg_other).map(i64::clone).unwrap();
                    registers.insert(reg, reg_other_value);
                },
                S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let prev_reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();
                    let new_reg_value =
                        operate(binop, prev_reg_value, int32 as i64);
                    registers.insert(reg, new_reg_value);
                },
                S::set_reg_binop_reg_reg {
                    reg,
                    binop,
                    reg_other,
                } => {
                    let prev_reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();
                    let reg_other_value =
                        registers.get(&reg_other).map(i64::clone).unwrap();
                    let new_reg_value =
                        operate(binop, prev_reg_value, reg_other_value);
                    registers.insert(reg, new_reg_value);
                },
            }
        }

        fn interpret_p(p: P, registers: &mut HashMap<Regs, i64>) -> i64 {
            match p {
                P::begin { ss } => {
                    ss.into_iter().for_each(|s| interpret_s(s, registers));
                    registers.get(&Regs::rax).map(i64::clone).unwrap()
                },
            }
        }

        interpret_p(self.p, &mut registers)
    }

    fn check_init(self) -> Result<Self, String> {
        let mut initialized_registers = HashSet::<Regs>::new();

        fn check_init_s(
            s: &S,
            initialized_registers: &mut HashSet<Regs>,
        ) -> Result<(), String> {
            match s {
                S::set_reg_int64 { reg, .. } => {
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                S::set_reg_reg { reg, reg_other } => {
                    let reg_other_is_initialized =
                        initialized_registers.contains(&reg_other);
                    match reg_other_is_initialized {
                        true => {
                            initialized_registers.insert(*reg);
                            Ok(())
                        },
                        false => Err(format!(
                            "{:?} is not initialized",
                            reg_other
                        )),
                    }
                },
                S::set_reg_binop_reg_int32 { reg, .. } => {
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                S::set_reg_binop_reg_reg { reg, reg_other, .. } => {
                    let reg_other_is_initialized =
                        initialized_registers.contains(&reg_other);
                    match reg_other_is_initialized {
                        true => {
                            initialized_registers.insert(*reg);
                            Ok(())
                        },
                        false => Err(format!(
                            "{:?} is not initialized",
                            reg_other
                        )),
                    }
                },
            }
        }

        fn check_init_p(
            p: &P,
            initialized_registers: &mut HashSet<Regs>,
        ) -> Result<(), String> {
            match p {
                P::begin { ref ss } => {
                    let errors = ss
                        .iter()
                        .filter_map(|s| {
                            let result =
                                check_init_s(s, initialized_registers);
                            match result {
                                Ok(()) => None,
                                Err(err) => Some(err),
                            }
                        })
                        .collect::<Vec<_>>();

                    match errors.first() {
                        Some(error) => Err(error),
                        None => Ok(()),
                    }?;

                    let rax_is_initialized =
                        initialized_registers.contains(&Regs::rax);

                    match rax_is_initialized {
                        // true => Ok(Paren_x64 { p }),
                        true => Ok(()),
                        false => Err(format!(
                            "{:?} is not initialized",
                            Regs::rax
                        )),
                    }
                },
            }
        }

        check_init_p(&self.p, &mut initialized_registers)?;
        Ok(self)
    }

    pub fn check(self) -> Result<Self, String> {
        self.check_init()
    }
}

impl ToString for Paren_x64 {
    fn to_string(&self) -> String {
        fn generate_binop(binop: &Binop) -> String {
            match binop {
                Binop::plus => format!("add"),
                Binop::multiply => format!("imul"),
            }
        }

        fn generate_s(s: &S) -> String {
            match s {
                S::set_reg_int64 { reg, int64 } => {
                    format!("mov {:?}, {}", reg, int64)
                },
                S::set_reg_reg { reg, reg_other } => {
                    format!("mov {:?}, {:?}", reg, reg_other)
                },
                S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let binop_as_string = generate_binop(binop);
                    format!("{:?} {:?}, {:?}", binop_as_string, reg, int32)
                },
                S::set_reg_binop_reg_reg {
                    reg,
                    binop,
                    reg_other,
                } => {
                    let binop_as_string = generate_binop(binop);
                    format!(
                        "{:?} {:?}, {:?}",
                        binop_as_string, reg, reg_other
                    )
                },
            }
        }

        fn generate_p(p: &P) -> String {
            match p {
                P::begin { ref ss } => {
                    ss.iter().fold(String::new(), |acc, s| {
                        let s_as_string = generate_s(s);
                        format!("{}\n{}", acc, s_as_string)
                    })
                },
            }
        }

        generate_p(&self.p)
    }
}

#[allow(non_camel_case_types)]
pub enum P {
    begin { ss: Vec<S> },
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

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}

#[cfg(test)]
mod tests;
