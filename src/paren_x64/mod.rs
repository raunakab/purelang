pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

use crate::cpsc411::Binop;
use crate::cpsc411::Check;
use crate::cpsc411::Interpret;
use crate::cpsc411::Reg;
use crate::paren_x64::data::Addr;
use crate::paren_x64::data::Loc;
use crate::paren_x64::data::Triv;
use crate::paren_x64::data::P;
use crate::paren_x64::data::S;

pub struct ParenX64 {
    p: P,
}

/// Check ParenX64 to make sure it's a valid ParenX64 program.
/// - Need to assert registers are initialized before using.
impl Check for ParenX64 {
    fn check(self) -> Result<Self, String> {
        self.check_init()
    }
}

/// Interpret ParenX64 to a value.
impl Interpret for ParenX64 {
    type Output = i64;

    fn interpret(self) -> Self::Output {
        let mut registers = HashMap::<Reg, i64>::new();
        let mut addrs = HashMap::<Addr, i64>::new();

        fn operate(binop: Binop, int64_1: i64, int64_2: i64) -> i64 {
            match binop {
                Binop::plus => int64_1 + int64_2,
                Binop::multiply => int64_1 * int64_2,
            }
        }

        fn get_value_in_loc(
            loc: Loc,
            registers: &HashMap<Reg, i64>,
            addrs: &HashMap<Addr, i64>,
        ) -> i64 {
            match loc {
                Loc::addr { addr } => addrs.get(&addr).map(i64::clone).unwrap(),
                Loc::reg { reg } => {
                    registers.get(&reg).map(i64::clone).unwrap()
                },
            }
        }

        fn get_value_in_triv(
            triv: Triv,
            registers: &HashMap<Reg, i64>,
            _: &HashMap<Addr, i64>,
        ) -> i64 {
            match triv {
                Triv::int64 { int64 } => int64,
                Triv::reg { reg } => {
                    registers.get(&reg).map(i64::clone).unwrap()
                },
            }
        }

        fn interpret_s(
            s: S,
            registers: &mut HashMap<Reg, i64>,
            addrs: &mut HashMap<Addr, i64>,
        ) {
            match s {
                S::set_addr_int32 { addr, int32 } => {
                    addrs.insert(addr, int32 as i64);
                },
                S::set_addr_reg { addr, reg } => {
                    let reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();

                    addrs.insert(addr, reg_value);
                },
                S::set_reg_loc { reg, loc } => {
                    let value_in_loc = get_value_in_loc(loc, registers, addrs);
                    registers.insert(reg, value_in_loc);
                },
                S::set_reg_triv { reg, triv } => {
                    let value_in_triv =
                        get_value_in_triv(triv, registers, addrs);
                    registers.insert(reg, value_in_triv);
                },
                S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let prev_reg_value =
                        registers.get(&reg).map(i64::clone).unwrap();
                    let new_reg_value =
                        operate(binop, prev_reg_value, int32 as i64);
                    registers.insert(reg, new_reg_value);
                },
                S::set_reg_binop_reg_loc { reg, binop, loc } => {
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
            p: P,
            registers: &mut HashMap<Reg, i64>,
            addrs: &mut HashMap<Addr, i64>,
        ) -> i64 {
            match p {
                P::begin { ss } => {
                    ss.into_iter()
                        .for_each(|s| interpret_s(s, registers, addrs));
                    registers.get(&Reg::rax).map(i64::clone).unwrap()
                },
            }
        }

        interpret_p(self.p, &mut registers, &mut addrs)
    }
}

/// Generate X64 source code in string form.
impl ToString for ParenX64 {
    fn to_string(&self) -> String {
        fn generate_binop(binop: &Binop) -> String {
            match binop {
                Binop::plus => format!("add"),
                Binop::multiply => format!("imul"),
            }
        }

        fn generate_addr(Addr { fbp, disp_offset }: &Addr) -> String {
            format!("QWORD [{:?} - {}]", fbp, disp_offset)
        }

        fn generate_loc(loc: &Loc) -> String {
            match loc {
                Loc::addr { addr } => generate_addr(addr),
                Loc::reg { reg } => format!("{:?}", reg),
            }
        }

        fn generate_triv(triv: &Triv) -> String {
            match triv {
                Triv::reg { reg } => format!("{:?}", reg),
                Triv::int64 { int64 } => format!("{}", int64),
            }
        }

        fn generate_s(s: &S) -> String {
            match s {
                S::set_addr_int32 { addr, int32 } => {
                    let addr_as_string = generate_addr(addr);
                    format!("mov {}, {}", addr_as_string, int32)
                },
                S::set_addr_reg { addr, reg } => {
                    let addr_as_string = generate_addr(addr);
                    format!("mov {}, {:?}", addr_as_string, reg)
                },
                S::set_reg_loc { reg, loc } => {
                    let loc_as_string = generate_loc(loc);
                    format!("mov {:?}, {}", reg, loc_as_string)
                },
                S::set_reg_triv { reg, triv } => {
                    let triv_as_string = generate_triv(triv);
                    format!("mov {:?}, {:?}", reg, triv_as_string)
                },
                S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let binop_as_string = generate_binop(binop);
                    format!("{:?} {:?}, {:?}", binop_as_string, reg, int32)
                },
                S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let binop_as_string = generate_binop(binop);
                    let loc_as_string = generate_loc(loc);
                    format!(
                        "{:?} {:?}, {:?}",
                        binop_as_string, reg, loc_as_string
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

impl ParenX64 {
    fn check_init(self) -> Result<Self, String> {
        let mut initialized_registers = HashSet::<Reg>::new();

        fn check_init_reg(
            reg: &Reg,
            initialized_registers: &HashSet<Reg>,
        ) -> Result<(), String> {
            let reg_is_initialized = initialized_registers.contains(reg);
            match reg_is_initialized {
                true => Ok(()),
                false => Err(format!("{:?} is not initialized", reg)),
            }
        }

        fn check_init_triv(
            triv: &Triv,
            initialized_registers: &HashSet<Reg>,
        ) -> Result<(), String> {
            match triv {
                Triv::int64 { .. } => Ok(()),
                Triv::reg { reg } => check_init_reg(reg, initialized_registers),
            }
        }

        fn check_init_loc(
            loc: &Loc,
            initialized_registers: &HashSet<Reg>,
        ) -> Result<(), String> {
            match loc {
                Loc::addr { .. } => Ok(()),
                Loc::reg { reg } => check_init_reg(reg, initialized_registers),
            }
        }

        fn check_init_s(
            s: &S,
            initialized_registers: &mut HashSet<Reg>,
        ) -> Result<(), String> {
            match s {
                S::set_addr_int32 { .. } => Ok(()),
                S::set_addr_reg { .. } => Ok(()),
                S::set_reg_loc { reg, loc } => {
                    check_init_loc(loc, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                S::set_reg_triv { reg, triv } => {
                    check_init_triv(triv, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                S::set_reg_binop_reg_int32 { reg, .. } => {
                    initialized_registers.insert(*reg);
                    Ok(())
                },
                S::set_reg_binop_reg_loc { reg, loc, .. } => {
                    check_init_loc(loc, initialized_registers)?;
                    initialized_registers.insert(*reg);
                    Ok(())
                },
            }
        }

        fn check_init_p(
            p: &P,
            initialized_registers: &mut HashSet<Reg>,
        ) -> Result<(), String> {
            match p {
                P::begin { ref ss } => {
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
                        initialized_registers.contains(&Reg::rax);

                    match rax_is_initialized {
                        // true => Ok(Paren_x64 { p }),
                        true => Ok(()),
                        false => {
                            Err(format!("{:?} is not initialized", Reg::rax))
                        },
                    }
                },
            }
        }

        check_init_p(&self.p, &mut initialized_registers)?;
        Ok(self)
    }
}
