pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

pub use self::data::*;
use crate::cpsc411;

pub struct ParenX64 {
    pub p: self::P,
}

/// Check ParenX64 to make sure it's a valid ParenX64 program.
/// - Need to assert registers are initialized before using.
impl cpsc411::Check for ParenX64 {
    fn check(self) -> Result<Self, String> {
        self.check_init()
    }
}

/// Interpret ParenX64 to a value.
impl cpsc411::Interpret for ParenX64 {
    type Output = i64;

    fn interpret(self) -> Self::Output {
        let mut registers = HashMap::<cpsc411::Reg, i64>::new();
        let mut addrs = HashMap::<self::Addr, i64>::new();

        fn operate(binop: cpsc411::Binop, int64_1: i64, int64_2: i64) -> i64 {
            match binop {
                cpsc411::Binop::plus => int64_1 + int64_2,
                cpsc411::Binop::multiply => int64_1 * int64_2,
            }
        }

        fn get_value_in_loc(
            loc: self::Loc,
            registers: &HashMap<cpsc411::Reg, i64>,
            addrs: &HashMap<self::Addr, i64>,
        ) -> i64 {
            match loc {
                self::Loc::addr { addr } => {
                    addrs.get(&addr).map(i64::clone).unwrap()
                },
                self::Loc::reg { reg } => {
                    registers.get(&reg).map(i64::clone).unwrap()
                },
            }
        }

        fn get_value_in_triv(
            triv: self::Triv,
            registers: &HashMap<cpsc411::Reg, i64>,
            _: &HashMap<self::Addr, i64>,
        ) -> i64 {
            match triv {
                self::Triv::int64 { int64 } => int64,
                self::Triv::reg { reg } => {
                    registers.get(&reg).map(i64::clone).unwrap()
                },
            }
        }

        fn interpret_s(
            s: self::S,
            registers: &mut HashMap<cpsc411::Reg, i64>,
            addrs: &mut HashMap<self::Addr, i64>,
        ) {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    addrs.insert(addr, int32 as i64);
                },
                self::S::set_addr_reg { addr, reg } => {
                    let reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();

                    addrs.insert(addr, reg_value);
                },
                self::S::set_reg_loc { reg, loc } => {
                    let value_in_loc = get_value_in_loc(loc, registers, addrs);
                    registers.insert(reg, value_in_loc);
                },
                self::S::set_reg_triv { reg, triv } => {
                    let value_in_triv =
                        get_value_in_triv(triv, registers, addrs);
                    registers.insert(reg, value_in_triv);
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let prev_reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();
                    let new_reg_value =
                        operate(binop, prev_reg_value, int32 as i64);
                    registers.insert(reg, new_reg_value);
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let prev_reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();
                    let value_in_loc = get_value_in_loc(loc, registers, addrs);
                    let new_reg_value =
                        operate(binop, prev_reg_value, value_in_loc);
                    registers.insert(reg, new_reg_value);
                },
            }
        }

        fn interpret_p(
            p: self::P,
            registers: &mut HashMap<cpsc411::Reg, i64>,
            addrs: &mut HashMap<self::Addr, i64>,
        ) -> i64 {
            match p {
                self::P::begin { ss } => {
                    ss.into_iter()
                        .for_each(|s| interpret_s(s, registers, addrs));
                    registers.get(&cpsc411::Reg::rax).map(i64::clone).unwrap()
                },
            }
        }

        interpret_p(self.p, &mut registers, &mut addrs)
    }
}

/// Generate X64 source code in string form.
impl ToString for ParenX64 {
    fn to_string(&self) -> String {
        fn generate_binop(binop: &cpsc411::Binop) -> String {
            match binop {
                cpsc411::Binop::plus => format!("add"),
                cpsc411::Binop::multiply => format!("imul"),
            }
        }

        fn generate_addr(Addr { fbp, disp_offset }: &self::Addr) -> String {
            format!("QWORD [{:?} - {}]", fbp, disp_offset)
        }

        fn generate_loc(loc: &self::Loc) -> String {
            match loc {
                self::Loc::addr { addr } => generate_addr(addr),
                self::Loc::reg { reg } => format!("{:?}", reg),
            }
        }

        fn generate_triv(triv: &self::Triv) -> String {
            match triv {
                self::Triv::reg { reg } => format!("{:?}", reg),
                self::Triv::int64 { int64 } => format!("{}", int64),
            }
        }

        fn generate_s(s: &self::S) -> String {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    let addr_as_string = generate_addr(addr);
                    format!("mov {}, {}", addr_as_string, int32)
                },
                self::S::set_addr_reg { addr, reg } => {
                    let addr_as_string = generate_addr(addr);
                    format!("mov {}, {:?}", addr_as_string, reg)
                },
                self::S::set_reg_loc { reg, loc } => {
                    let loc_as_string = generate_loc(loc);
                    format!("mov {:?}, {}", reg, loc_as_string)
                },
                self::S::set_reg_triv { reg, triv } => {
                    let triv_as_string = generate_triv(triv);
                    format!("mov {:?}, {:?}", reg, triv_as_string)
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let binop_as_string = generate_binop(binop);
                    format!("{:?} {:?}, {:?}", binop_as_string, reg, int32)
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let binop_as_string = generate_binop(binop);
                    let loc_as_string = generate_loc(loc);
                    format!(
                        "{:?} {:?}, {:?}",
                        binop_as_string, reg, loc_as_string
                    )
                },
            }
        }

        fn generate_p(p: &self::P) -> String {
            match p {
                self::P::begin { ref ss } => {
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

impl ParenX64 {
    fn check_init(self) -> Result<Self, String> {
        let mut initialized_registers = HashSet::<cpsc411::Reg>::new();

        fn check_init_reg(
            reg: &cpsc411::Reg,
            initialized_registers: &HashSet<cpsc411::Reg>,
        ) -> Result<(), String> {
            let reg_is_initialized = initialized_registers.contains(reg);
            match reg_is_initialized {
                true => Ok(()),
                false => Err(format!("{:?} is not initialized", reg)),
            }
        }

        fn check_init_triv(
            triv: &self::Triv,
            initialized_registers: &HashSet<cpsc411::Reg>,
        ) -> Result<(), String> {
            match triv {
                self::Triv::int64 { .. } => Ok(()),
                self::Triv::reg { reg } => {
                    check_init_reg(reg, initialized_registers)
                },
            }
        }

        fn check_init_loc(
            loc: &self::Loc,
            initialized_registers: &HashSet<cpsc411::Reg>,
        ) -> Result<(), String> {
            match loc {
                self::Loc::addr { .. } => Ok(()),
                self::Loc::reg { reg } => {
                    check_init_reg(reg, initialized_registers)
                },
            }
        }

        fn check_init_s(
            s: &self::S,
            initialized_registers: &mut HashSet<cpsc411::Reg>,
        ) -> Result<(), String> {
            match s {
                self::S::set_addr_int32 { .. } => Ok(()),
                self::S::set_addr_reg { .. } => Ok(()),
                self::S::set_reg_loc { reg, loc } => {
                    check_init_loc(loc, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                self::S::set_reg_triv { reg, triv } => {
                    check_init_triv(triv, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                self::S::set_reg_binop_reg_int32 { reg, .. } => {
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                self::S::set_reg_binop_reg_loc { reg, loc, .. } => {
                    check_init_loc(loc, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
            }
        }

        fn check_init_p(
            p: &self::P,
            initialized_registers: &mut HashSet<cpsc411::Reg>,
        ) -> Result<(), String> {
            match p {
                self::P::begin { ref ss } => {
                    let errors = ss
                        .iter()
                        .filter_map(|s| {
                            let result = check_init_s(s, initialized_registers);
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
                        initialized_registers.contains(&cpsc411::Reg::rax);

                    match rax_is_initialized {
                        // true => Ok(Paren_x64 { p }),
                        true => Ok(()),
                        false => Err(format!(
                            "{:?} is not initialized",
                            cpsc411::Reg::rax
                        )),
                    }
                },
            }
        }

        check_init_p(&self.p, &mut initialized_registers)?;
        Ok(self)
    }
}
