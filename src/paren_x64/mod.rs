pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

pub use self::data::*;
use crate::cpsc411;
use crate::paren_x64_rt as target;

#[derive(Clone)]
pub struct ParenX64(pub self::P);

impl ParenX64 {
    /// ### Purpose:
    /// Ensure all labels are unique and all jumps reference an existing label.
    fn check_labels(self) -> Result<Self, String> {
        type LabelStore = HashSet<cpsc411::Label>;

        let Self(p) = &self;

        fn check_p(p: &self::P) -> Result<(), String> {
            let mut labels = LabelStore::default();

            match p {
                self::P::begin(ss) => {
                    ss.iter()
                        .try_for_each(|s| collect_labels(s, &mut labels))?;

                    ss.iter().try_for_each(|s| check_jumps(s, &labels))?;

                    Ok(())
                },
            }
        }

        fn collect_labels(
            s: &self::S,
            labels: &mut LabelStore,
        ) -> Result<(), String> {
            match s {
                self::S::with_label { label, .. } => {
                    let is_a_new_label = labels.insert(label.clone());

                    match is_a_new_label {
                        true => Ok(()),
                        false => {
                            let error_msg = format!(
                                "The label, '{:?}', already exists.",
                                label
                            );
                            Err(error_msg)
                        },
                    }
                },
                _ => Ok(()),
            }
        }

        fn check_jumps(s: &self::S, labels: &LabelStore) -> Result<(), String> {
            match s {
                self::S::jump_trg(trg) => check_trg(trg, labels),
                self::S::compare_reg_opand_jump_if { label, .. } => {
                    check_label(label, labels)
                },
                _ => Ok(()),
            }
        }

        fn check_trg(
            trg: &self::Trg,
            labels: &LabelStore,
        ) -> Result<(), String> {
            match trg {
                self::Trg::reg(..) => Ok(()),
                self::Trg::label(label) => check_label(label, labels),
            }
        }

        fn check_label(
            label: &cpsc411::Label,
            labels: &LabelStore,
        ) -> Result<(), String> {
            let label_found = labels.contains(label);

            match label_found {
                true => Ok(()),
                false => {
                    let error_msg =
                        format!("The label, '{:?}', was not found.", label);
                    Err(error_msg)
                },
            }
        }

        check_p(&p)?;
        Ok(self)
    }

    /// ### Purpose:
    /// Generate X64 source code in string form.
    pub fn generate_x64(self) -> String {
        let Self(p) = self;

        fn generate_p(p: &self::P) -> String {
            match p {
                self::P::begin(ref ss) => {
                    ss.iter().fold(String::new(), |acc, s| {
                        let s_as_string = generate_s(s);
                        format!("{}\n{}", acc, s_as_string)
                    })
                },
            }
        }

        fn generate_s(s: &self::S) -> String {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    let addr = generate_addr(addr);
                    format!("mov {}, {}", addr, int32)
                },
                self::S::set_addr_trg { addr, trg } => {
                    let addr = generate_addr(addr);
                    let trg = generate_trg(trg);

                    format!("mov {}, {}", addr, trg)
                },
                self::S::set_reg_loc { reg, loc } => {
                    let loc = generate_loc(loc);
                    format!("mov {:?}, {}", reg, loc)
                },
                self::S::set_reg_triv { reg, triv } => {
                    let triv = generate_triv(triv);
                    format!("mov {:?}, {}", reg, triv)
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let binop = generate_binop(binop);
                    format!("{} {:?}, {}", binop, reg, int32)
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let binop = generate_binop(binop);
                    let loc = generate_loc(loc);

                    format!("{} {:?}, {}", binop, reg, loc)
                },
                self::S::with_label { label, s } => {
                    let label = generate_label(label);
                    let s = generate_s(&s);

                    format!("{}:\n{}", label, s)
                },
                self::S::jump_trg(trg) => {
                    let trg = generate_trg(trg);
                    format!("jmp {}", trg)
                },
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                } => {
                    let opand = generate_opand(opand);
                    let cmp_instr = format!("cmp {:?}, {}", reg, opand);

                    let relop = generate_relop(relop);
                    let label = generate_label(label);
                    let jmp_instr = format!("{} {}", relop, label);

                    format!("{}\n{}", cmp_instr, jmp_instr)
                },
                self::S::nop => "".into(),
            }
        }

        fn generate_triv(triv: &self::Triv) -> String {
            match triv {
                self::Triv::trg(trg) => format!("{:?}", trg),
                self::Triv::int64(int64) => format!("{}", int64),
            }
        }

        fn generate_label(label: &cpsc411::Label) -> String {
            let cpsc411::Label(label) = label.clone();

            label
        }

        fn generate_trg(trg: &self::Trg) -> String {
            match trg {
                self::Trg::reg(reg) => format!("{:#?}", reg),
                self::Trg::label(label) => generate_label(label),
            }
        }

        fn generate_addr(
            cpsc411::Addr { fbp, disp_offset }: &cpsc411::Addr,
        ) -> String {
            format!("QWORD [{:?} - {}]", fbp, disp_offset)
        }

        fn generate_opand(opand: &self::Opand) -> String {
            match opand {
                self::Opand::int64(int64) => format!("{}", int64),
                self::Opand::reg(reg) => format!("{:#?}", reg),
            }
        }

        fn generate_loc(loc: &self::Loc) -> String {
            match loc {
                self::Loc::addr(addr) => generate_addr(addr),
                self::Loc::reg(reg) => format!("{:?}", reg),
            }
        }

        fn generate_binop(binop: &cpsc411::Binop) -> String {
            match binop {
                cpsc411::Binop::plus => "add",
                cpsc411::Binop::multiply => "imul",
            }
            .into()
        }

        fn generate_relop(relop: &cpsc411::Relop) -> String {
            match relop {
                cpsc411::Relop::gt => "jg",
                cpsc411::Relop::gte => "jge",
                cpsc411::Relop::lt => "jl",
                cpsc411::Relop::lte => "jle",
                cpsc411::Relop::eq => "je",
                cpsc411::Relop::neq => "jne",
            }
            .into()
        }

        generate_p(&p)
    }

    /// ### Purpose:
    /// Compiles Paren-x64 v4 to Paren-x64-rt v4 by resolving all labels to
    /// their position in the instruction sequence.
    pub fn link_paren_x64(self) -> target::ParenX64Rt {
        type LabelStore = HashMap<cpsc411::Label, cpsc411::PcAddr>;

        let Self(p) = self;

        fn link_p(p: self::P) -> target::P {
            let mut labels = LabelStore::default();

            match p {
                self::P::begin(ss) => {
                    ss.iter().enumerate().for_each(|(curr_index, s)| {
                        resolve_labels(s, &mut labels, curr_index);
                    });

                    let ss =
                        ss.into_iter().map(|s| link_s(s, &labels)).collect();

                    target::P::begin(ss)
                },
            }
        }

        fn resolve_labels(
            s: &self::S,
            labels: &mut LabelStore,
            curr_index: cpsc411::PcAddr,
        ) {
            match s {
                self::S::with_label { label, .. } => {
                    let prev_pc_addr = labels.insert(label.clone(), curr_index);

                    match prev_pc_addr {
                        Some(_) => {
                            let cpsc411::Label(label) = label;

                            panic!("The label, '{}', already exists.", label);
                        },
                        None => (),
                    }
                },
                _ => (),
            }
        }

        fn link_s(s: self::S, labels: &LabelStore) -> target::S {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    target::S::set_addr_int32 { addr, int32 }
                },
                self::S::set_addr_trg { addr, trg } => {
                    let trg = link_trg(trg, labels);
                    target::S::set_addr_trg { addr, trg }
                },
                self::S::set_reg_loc { reg, loc } => target::S::set_reg_loc {
                    reg,
                    loc: loc.into(),
                },
                self::S::set_reg_triv { reg, triv } => {
                    let triv = link_triv(triv, labels);
                    target::S::set_reg_triv { reg, triv }
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    target::S::set_reg_binop_reg_int32 { reg, binop, int32 }
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    target::S::set_reg_binop_reg_loc {
                        reg,
                        binop,
                        loc: loc.into(),
                    }
                },
                self::S::with_label { s, .. } => {
                    let s = *s;
                    link_s(s, labels)
                },
                self::S::jump_trg(trg) => {
                    let trg = link_trg(trg, labels);
                    target::S::jump_trg(trg)
                },
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                } => {
                    let pc_addr = *labels.get(&label).unwrap();

                    target::S::compare_reg_opand_jump_if {
                        reg,
                        opand: opand.into(),
                        relop,
                        pc_addr,
                    }
                },
                self::S::nop => target::S::nop,
            }
        }

        fn link_triv(triv: self::Triv, labels: &LabelStore) -> target::Triv {
            match triv {
                self::Triv::int64(int64) => target::Triv::int64(int64),
                self::Triv::trg(trg) => {
                    let trg = link_trg(trg, labels);
                    target::Triv::trg(trg)
                },
            }
        }

        fn link_trg(trg: self::Trg, labels: &LabelStore) -> target::Trg {
            match trg {
                self::Trg::reg(reg) => target::Trg::reg(reg),
                self::Trg::label(label) => {
                    let pc_addr = *labels.get(&label).unwrap();
                    target::Trg::pc_addr(pc_addr)
                },
            }
        }

        let p = link_p(p);

        target::ParenX64Rt(p)
    }
}

/// ### Purpose:
/// Check ParenX64 to make sure it's a valid ParenX64 program.
/// - Need to ensure all labels are unique.
/// - Need to ensure all jumps reference an existing label.
///
/// ### Notes:
/// In general, it cannot be checked that a register is initialized before
/// usage. This is due to ParenX64 allowing for "control-flow", some of which
/// may be dynamic and unable to be checked at compile-time.
impl cpsc411::Check for ParenX64 {
    fn check(self) -> Result<Self, String> {
        self.check_labels()
    }
}

/// ### Purpose:
/// Interpret ParenX64 source code into an i64.
impl cpsc411::Interpret for ParenX64 {
    type Output = i64;

    fn interpret(self) -> Self::Output {
        self.link_paren_x64().interpret()
    }
}
