pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

pub use self::data::*;
use crate::cpsc411;
use crate::paren_x64_rt as target;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ParenX64(pub self::P);

impl ParenX64 {
    /// ### Purpose:
    /// Ensure all labels are unique and all jumps reference an existing label.
    pub fn check_labels(self) -> Result<Self, String> {
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
                self::P::begin(ref ss) => ss.iter().enumerate().fold(
                    String::new(),
                    |acc, (index, s)| {
                        let s = generate_s(s);

                        match index {
                            0 => s,
                            _ => format!("{}\n{}", acc, s),
                        }
                    },
                ),
            }
        }

        fn generate_s(s: &self::S) -> String {
            match s {
                self::S::set_addr_int32 { addr, int32 } => {
                    let addr = generate_addr(addr);

                    format!("\tmov {}, {}", addr, int32)
                },
                self::S::set_addr_trg { addr, trg } => {
                    let addr = generate_addr(addr);

                    let trg = generate_trg(trg);

                    format!("\tmov {}, {}", addr, trg)
                },
                self::S::set_reg_loc { reg, loc } => {
                    let loc = generate_loc(loc);

                    format!("\tmov {:?}, {}", reg, loc)
                },
                self::S::set_reg_triv { reg, triv } => {
                    let triv = generate_triv(triv);

                    format!("\tmov {:?}, {}", reg, triv)
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    let binop = generate_binop(binop);

                    format!("\t{} {:?}, {}", binop, reg, int32)
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let binop = generate_binop(binop);

                    let loc = generate_loc(loc);

                    format!("\t{} {:?}, {}", binop, reg, loc)
                },
                self::S::with_label { label, s } => {
                    let label = generate_label(label);

                    let s = generate_s(&s);

                    format!("{}:\n{}", label, s)
                },
                self::S::jump_trg(trg) => {
                    let trg = generate_trg(trg);

                    format!("\tjmp {}", trg)
                },
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                } => {
                    let opand = generate_opand(opand);

                    let cmp_instr = format!("\tcmp {:?}, {}", reg, opand);

                    let relop = generate_relop(relop);

                    let label = generate_label(label);

                    let jmp_instr = format!("\t{} {}", relop, label);

                    format!("{}\n{}", cmp_instr, jmp_instr)
                },
                self::S::nop => "".into(),
            }
        }

        fn generate_triv(triv: &self::Triv) -> String {
            match triv {
                self::Triv::trg(trg) => generate_trg(trg),
                self::Triv::int64(int64) => int64.to_string(),
            }
        }

        fn generate_label(cpsc411::Label(label): &cpsc411::Label) -> String {
            label.clone()
        }

        fn generate_trg(trg: &self::Trg) -> String {
            match trg {
                self::Trg::reg(reg) => generate_reg(reg),
                self::Trg::label(label) => generate_label(label),
            }
        }

        fn generate_reg(reg: &cpsc411::Reg) -> String {
            format!("{:#?}", reg)
        }

        fn generate_addr(
            cpsc411::Addr { fbp, disp_offset }: &cpsc411::Addr,
        ) -> String {
            let fbp = generate_reg(fbp);

            format!("QWORD [{} - {}]", fbp, disp_offset)
        }

        fn generate_opand(opand: &self::Opand) -> String {
            match opand {
                self::Opand::int64(int64) => int64.to_string(),
                self::Opand::reg(reg) => generate_reg(reg),
            }
        }

        fn generate_loc(loc: &self::Loc) -> String {
            match loc {
                self::Loc::addr(addr) => generate_addr(addr),
                self::Loc::reg(reg) => generate_reg(reg),
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
            let label_store = LabelStore::default();

            match p {
                self::P::begin(ss) => {
                    let label_store = ss.iter().enumerate().fold(
                        label_store,
                        |label_store, (index, s)| {
                            resolve_labels(s, label_store, index)
                        },
                    );

                    let ss = ss
                        .into_iter()
                        .map(|s| link_s(s, &label_store))
                        .collect();

                    target::P::begin(ss)
                },
            }
        }

        fn resolve_labels(
            s: &self::S,
            mut label_store: LabelStore,
            curr_index: cpsc411::PcAddr,
        ) -> LabelStore {
            match s {
                self::S::with_label { label, .. } => {
                    let prev_pc_addr =
                        label_store.insert(label.clone(), curr_index);

                    match prev_pc_addr {
                        Some(_) => {
                            let cpsc411::Label(label) = label;

                            panic!("The label, '{}', already exists.", label);
                        },
                        None => label_store,
                    }
                },
                _ => label_store,
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
