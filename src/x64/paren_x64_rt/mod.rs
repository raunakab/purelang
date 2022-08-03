pub mod data;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::HashMap;

pub use self::data::*;
use crate::utils;

pub struct ParenX64Rt(pub self::P);

impl ParenX64Rt {
    /// ### Purpose:
    /// Interpret the ParenX64Rt program as a value, returning the final value
    /// of rax.
    pub fn interp_loop(self) -> i64 {
        type RegEnv = HashMap<utils::Reg, i64>;

        type AddrEnv = HashMap<utils::Addr, i64>;

        enum Control {
            next,
            jump { pc_addr: utils::PcAddr },
        }

        let Self(p) = self;

        fn interp_p(p: self::P) -> i64 {
            let mut reg_env = RegEnv::default();

            let mut addr_env = AddrEnv::default();

            let mut pc_addr = utils::PcAddr::default();

            match p {
                self::P::begin(ss) => {
                    let max_pc_addr = ss.len();

                    loop {
                        match pc_addr.cmp(&max_pc_addr) {
                            Ordering::Less => (),
                            Ordering::Equal | Ordering::Greater => break,
                        };

                        let s = ss.get(pc_addr).unwrap();

                        let control = interp_s(s, &mut reg_env, &mut addr_env);
                        match control {
                            Control::next => pc_addr += 1usize,
                            Control::jump {
                                pc_addr: next_pc_addr,
                            } => pc_addr = next_pc_addr,
                        }
                    }

                    let return_reg = utils::Reg::current_return_reg();
                    reg_env.get(&return_reg).map(i64::clone).unwrap()
                },
            }
        }

        fn interp_s(
            s: &self::S,
            reg_env: &mut RegEnv,
            addr_env: &mut AddrEnv,
        ) -> Control {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    addr_env.insert(addr.clone(), *int32 as i64);
                    Control::next
                },
                self::S::set_addr_trg { addr, trg } => {
                    let value = get_from_trg(trg, reg_env);
                    addr_env.insert(addr.clone(), value);
                    Control::next
                },
                self::S::set_reg_triv { reg, triv } => {
                    let value = get_from_triv(triv, reg_env);
                    reg_env.insert(*reg, value);
                    Control::next
                },
                self::S::set_reg_loc { reg, loc } => {
                    let value = get_from_loc(loc, reg_env, addr_env);
                    reg_env.insert(*reg, value);
                    Control::next
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let value1 = get_from_reg(reg, reg_env);
                    let value2 = *int32 as i64;
                    let value = bin_operate(binop, value1, value2);

                    reg_env.insert(*reg, value);
                    Control::next
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let value1 = get_from_reg(reg, reg_env);
                    let value2 = get_from_loc(loc, reg_env, addr_env);
                    let value = bin_operate(binop, value1, value2);

                    reg_env.insert(*reg, value);
                    Control::next
                },
                self::S::jump_trg(trg) => {
                    let value: usize =
                        get_from_trg(trg, reg_env).try_into().unwrap();
                    Control::jump { pc_addr: value }
                },
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    pc_addr,
                } => {
                    let value1 = get_from_reg(reg, reg_env);
                    let value2 = get_from_opand(opand, reg_env);

                    let should_jump = rel_operate(relop, value1, value2);
                    let pc_addr = *pc_addr;
                    match should_jump {
                        true => Control::jump { pc_addr },
                        false => Control::next,
                    }
                },
                self::S::nop => Control::next,
            }
        }

        fn get_from_loc(
            loc: &self::Loc,
            reg_env: &RegEnv,
            addr_env: &AddrEnv,
        ) -> i64 {
            match loc {
                self::Loc::addr(addr) => get_from_addr(addr, addr_env),
                self::Loc::reg(reg) => get_from_reg(reg, reg_env),
            }
        }

        fn get_from_trg(trg: &self::Trg, reg_env: &RegEnv) -> i64 {
            match trg {
                self::Trg::pc_addr(pc_addr) => *pc_addr as i64,
                self::Trg::reg(reg) => get_from_reg(reg, reg_env),
            }
        }

        fn get_from_reg(reg: &utils::Reg, reg_env: &RegEnv) -> i64 {
            reg_env.get(reg).map(i64::clone).unwrap()
        }

        fn get_from_addr(addr: &utils::Addr, addr_env: &AddrEnv) -> i64 {
            addr_env.get(addr).map(i64::clone).unwrap()
        }

        fn get_from_triv(triv: &self::Triv, reg_env: &RegEnv) -> i64 {
            match triv {
                self::Triv::int64(int64) => *int64,
                self::Triv::trg(trg) => get_from_trg(trg, reg_env),
            }
        }

        fn get_from_opand(opand: &self::Opand, reg_env: &RegEnv) -> i64 {
            match opand {
                self::Opand::int64(int64) => *int64,
                self::Opand::reg(reg) => get_from_reg(reg, reg_env),
            }
        }

        fn bin_operate(
            binop: &utils::Binop,
            value1: i64,
            value2: i64,
        ) -> i64 {
            match binop {
                utils::Binop::plus => value1 + value2,
                utils::Binop::multiply => value1 * value2,
            }
        }

        fn rel_operate(
            relop: &utils::Relop,
            value1: i64,
            value2: i64,
        ) -> bool {
            match relop {
                utils::Relop::gt => value1 < value2,
                utils::Relop::gte => value1 <= value2,
                utils::Relop::lt => value1 > value2,
                utils::Relop::lte => value1 >= value2,
                utils::Relop::eq => value1 == value2,
                utils::Relop::neq => value1 != value2,
            }
        }

        interp_p(p)
    }
}
