pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::structured_control_flow::paren_x64_fvars as target;
use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ParaAsmLang(pub self::P);

impl ParaAsmLang {
    /// ### Purpose:
    /// Compiles Para-asm-lang v2 to Paren-x64-fvars v2 by patching instructions
    /// that have no x64 analogue into a sequence of instructions.
    ///
    /// ### Notes:
    /// The implementation should use auxiliary registers from
    /// current-patch-instructions-registers when generating instruction
    /// sequences, and current-return-value-register for compiling halt.
    pub fn patch_instructions(self) -> target::ParenX64Fvars {
        let Self(p) = self;

        fn patch_p(p: self::P) -> target::P {
            match p {
                self::P::begin(ss) => {
                    let mut ss = ss
                        .into_iter()
                        .map(patch_s)
                        .flatten()
                        .collect::<Vec<_>>();

                    let halt_label = utils::Label::halt_label();

                    let halt_instr = target::S::with_label {
                        label: halt_label,
                        s: Box::new(target::S::nop),
                    };

                    ss.push(halt_instr);

                    target::P::begin(ss)
                },
            }
        }

        fn patch_s(s: self::S) -> Vec<target::S> {
            match s {
                self::S::halt(opand) => {
                    let return_reg = utils::Reg::current_return_reg();

                    let instr1 = match opand {
                        self::Opand::int64(int64) => target::S::set_reg_triv {
                            reg: return_reg,
                            triv: target::Triv::int64(int64),
                        },
                        self::Opand::loc(loc) => target::S::set_reg_loc {
                            reg: return_reg,
                            loc,
                        },
                    };

                    let instr2 = target::S::jump(target::Trg::label(
                        utils::Label::halt_label(),
                    ));

                    vec![instr1, instr2]
                },

                self::S::set_loc_triv { loc, triv } => match (loc, triv) {
                    (self::Loc::reg(reg), self::Triv::opand(opand)) => {
                        match opand {
                            // reg <- int64
                            self::Opand::int64(int64) => {
                                let instr = target::S::set_reg_triv {
                                    reg,
                                    triv: target::Triv::int64(int64),
                                };

                                vec![instr]
                            },

                            // reg <- loc
                            self::Opand::loc(loc) => {
                                let instr = target::S::set_reg_loc { reg, loc };

                                vec![instr]
                            },
                        }
                    },

                    // reg <- label
                    (self::Loc::reg(reg), self::Triv::label(label)) => {
                        let instr = target::S::set_reg_triv {
                            reg,
                            triv: target::Triv::trg(target::Trg::label(label)),
                        };

                        vec![instr]
                    },

                    (self::Loc::fvar(fvar), self::Triv::opand(opand)) => {
                        match opand {
                            self::Opand::int64(int64) => {
                                i32::try_from(int64).ok().map_or_else(
                                    // reg <- int64
                                    // fvar <- reg
                                    || {
                                        let (aux_reg, _) = utils::Reg::current_auxiliary_registers();

                                        let instr1 = target::S::set_reg_triv { reg: aux_reg, triv: target::Triv::int64(int64) };

                                        let instr2 = target::S::set_fvar_trg { fvar, trg: target::Trg::reg(aux_reg) };

                                        vec![instr1, instr2]
                                    },

                                    // reg <- int32
                                    |int32| {
                                        let instr = target::S::set_fvar_int32 {
                                            fvar,
                                            int32,
                                        };

                                        vec![instr]
                                    },
                                )
                            },

                            self::Opand::loc(loc) => {
                                match loc {
                                    // fvar <- reg
                                    self::Loc::reg(reg) => {
                                        let instr = target::S::set_fvar_trg {
                                            fvar,
                                            trg: target::Trg::reg(reg),
                                        };

                                        vec![instr]
                                    },
                                    // reg <- fvar2
                                    // fvar <- reg
                                    self::Loc::fvar(fvar2) => {
                                        let (aux_reg, _) = utils::Reg::current_auxiliary_registers();

                                        let instr1 = target::S::set_reg_loc {
                                            reg: aux_reg,
                                            loc: target::Loc::fvar(fvar2),
                                        };

                                        let instr2 = target::S::set_fvar_trg {
                                            fvar,
                                            trg: target::Trg::reg(aux_reg),
                                        };

                                        vec![instr1, instr2]
                                    },
                                }
                            },
                        }
                    },

                    // fvar <- label
                    (self::Loc::fvar(fvar), self::Triv::label(label)) => {
                        let instr = target::S::set_fvar_trg {
                            fvar,
                            trg: target::Trg::label(label),
                        };

                        vec![instr]
                    },
                },

                self::S::set_loc_binop_opand { loc, binop, opand } => {
                    match (loc, opand) {
                        (self::Loc::reg(reg), self::Opand::int64(int64)) => {
                            i32::try_from(int64).ok().map_or_else(
                                // aux_reg <- int64
                                // reg <- reg + aux_reg
                                || {
                                    let (aux_reg, _) =
                                        utils::Reg::current_auxiliary_registers(
                                        );

                                    let instr1 = target::S::set_reg_triv {
                                        reg: aux_reg,
                                        triv: target::Triv::int64(int64),
                                    };

                                    let instr2 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg,
                                            binop,
                                            loc: target::Loc::reg(aux_reg),
                                        };

                                    vec![instr1, instr2]
                                },
                                // reg <- reg + loc
                                |int32| {
                                    let instr =
                                        target::S::set_reg_binop_reg_int32 {
                                            reg,
                                            binop,
                                            int32,
                                        };

                                    vec![instr]
                                },
                            )
                        },

                        // reg <- reg + loc
                        (self::Loc::reg(reg), self::Opand::loc(loc)) => {
                            let instr = target::S::set_reg_binop_reg_loc {
                                reg,
                                binop,
                                loc,
                            };

                            vec![instr]
                        },

                        (self::Loc::fvar(fvar), self::Opand::int64(int64)) => {
                            i32::try_from(int64).ok().map_or_else(
                                // aux_reg <- int64
                                // aux_reg' <- fvar
                                // aux_reg <- aux_reg + aux_reg'
                                // fvar <- aux_reg
                                || {
                                    let (aux_reg, aux_reg_2) =
                                        utils::Reg::current_auxiliary_registers(
                                        );

                                    let instr1 = target::S::set_reg_triv {
                                        reg: aux_reg,
                                        triv: target::Triv::int64(int64),
                                    };

                                    let instr2 = target::S::set_reg_loc {
                                        reg: aux_reg_2,
                                        loc: target::Loc::fvar(fvar),
                                    };

                                    let instr3 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg: aux_reg,
                                            binop,
                                            loc: target::Loc::reg(aux_reg_2),
                                        };

                                    let instr4 = target::S::set_fvar_trg {
                                        fvar,
                                        trg: target::Trg::reg(aux_reg),
                                    };

                                    vec![instr1, instr2, instr3, instr4]
                                },
                                // aux_reg <- fvar
                                // aux_reg <- aux_reg + int32
                                // fvar <- aux_reg
                                |int32| {
                                    let (aux_reg, _) =
                                        utils::Reg::current_auxiliary_registers(
                                        );

                                    let instr1 = target::S::set_reg_loc {
                                        reg: aux_reg,
                                        loc: target::Loc::fvar(fvar),
                                    };

                                    let instr2 =
                                        target::S::set_reg_binop_reg_int32 {
                                            reg: aux_reg,
                                            binop,
                                            int32,
                                        };

                                    let instr3 = target::S::set_fvar_trg {
                                        fvar,
                                        trg: target::Trg::reg(aux_reg),
                                    };

                                    vec![instr1, instr2, instr3]
                                },
                            )
                        },

                        (self::Loc::fvar(fvar), self::Opand::loc(loc)) => {
                            match loc {
                                // aux_reg <- fvar
                                // aux_reg <- aux_reg + reg
                                // fvar <- aux_reg
                                self::Loc::reg(reg) => {
                                    let (aux_reg, _) =
                                        utils::Reg::current_auxiliary_registers(
                                        );

                                    let instr1 = target::S::set_reg_loc {
                                        reg: aux_reg,
                                        loc: target::Loc::fvar(fvar),
                                    };

                                    let instr2 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg: aux_reg,
                                            binop,
                                            loc: target::Loc::reg(reg),
                                        };

                                    let instr3 = target::S::set_fvar_trg {
                                        fvar,
                                        trg: target::Trg::reg(aux_reg),
                                    };

                                    vec![instr1, instr2, instr3]
                                },

                                // aux_reg <- fvar
                                // aux_reg' <- fvar2
                                // aux_reg <- aux_reg + aux_reg'
                                // fvar <- aux_reg
                                self::Loc::fvar(fvar2) => {
                                    let (aux_reg, aux_reg_2) =
                                        utils::Reg::current_auxiliary_registers(
                                        );

                                    let instr1 = target::S::set_reg_loc {
                                        reg: aux_reg,
                                        loc: target::Loc::fvar(fvar),
                                    };

                                    let instr2 = target::S::set_reg_loc {
                                        reg: aux_reg_2,
                                        loc: target::Loc::fvar(fvar2),
                                    };

                                    let instr3 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg: aux_reg,
                                            binop,
                                            loc: target::Loc::reg(aux_reg_2),
                                        };

                                    let instr4 = target::S::set_fvar_trg {
                                        fvar,
                                        trg: target::Trg::reg(aux_reg),
                                    };

                                    vec![instr1, instr2, instr3, instr4]
                                },
                            }
                        },
                    }
                },

                self::S::jump(trg) => match trg {
                    self::Trg::label(label) => {
                        let instr = target::S::jump(target::Trg::label(label));

                        vec![instr]
                    },
                    self::Trg::loc(loc) => match loc {
                        self::Loc::reg(reg) => {
                            let instr = target::S::jump(target::Trg::reg(reg));

                            vec![instr]
                        },
                        self::Loc::fvar(fvar) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 =
                                target::S::jump(target::Trg::reg(aux_reg));

                            vec![instr1, instr2]
                        },
                    },
                },

                self::S::with_label { label, s } => patch_s(*s)
                    .into_iter()
                    .enumerate()
                    .map(|(index, s)| match index {
                        0usize => target::S::with_label {
                            label: label.clone(),
                            s: Box::new(s),
                        },
                        _ => s,
                    })
                    .collect(),

                self::S::compare_jump {
                    loc,
                    opand,
                    relop,
                    trg,
                } => match (loc, opand, trg) {
                    (
                        self::Loc::reg(reg),
                        self::Opand::int64(int64),
                        self::Trg::label(label),
                    ) => {
                        let instr = target::S::compare_reg_opand_jump_if {
                            reg,
                            opand: target::Opand::int64(int64),
                            relop,
                            label,
                        };

                        vec![instr]
                    },
                    (
                        self::Loc::reg(reg),
                        self::Opand::int64(int64),
                        self::Trg::loc(loc),
                    ) => match loc {
                        self::Loc::reg(reg) => {
                            let label = generate_neg_jump_label();

                            let instr1 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::int64(int64),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr2 = target::S::jump(target::Trg::reg(reg));

                            let instr3 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3]
                        },
                        self::Loc::fvar(fvar) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::int64(int64),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr2 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr3 =
                                target::S::jump(target::Trg::reg(aux_reg));

                            let instr4 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4]
                        },
                    },
                    (
                        self::Loc::reg(reg),
                        self::Opand::loc(loc),
                        self::Trg::label(label),
                    ) => match loc {
                        self::Loc::reg(reg2) => {
                            let instr = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(reg2),
                                relop,
                                label,
                            };

                            vec![instr]
                        },
                        self::Loc::fvar(fvar) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(aux_reg),
                                relop,
                                label,
                            };

                            vec![instr1, instr2]
                        },
                    },
                    (
                        self::Loc::reg(reg),
                        self::Opand::loc(loc),
                        self::Trg::loc(loc2),
                    ) => match (loc, loc2) {
                        (self::Loc::reg(reg2), self::Loc::reg(reg3)) => {
                            let label = generate_neg_jump_label();

                            let instr1 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(reg2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr2 =
                                target::S::jump(target::Trg::reg(reg3));

                            let instr3 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3]
                        },
                        (self::Loc::reg(reg2), self::Loc::fvar(fvar3)) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(reg2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr2 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar3),
                            };

                            let instr3 =
                                target::S::jump(target::Trg::reg(aux_reg));

                            let instr4 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4]
                        },
                        (self::Loc::fvar(fvar2), self::Loc::reg(reg3)) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar2),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(aux_reg),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 =
                                target::S::jump(target::Trg::reg(reg3));

                            let instr4 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4]
                        },
                        (self::Loc::fvar(fvar2), self::Loc::fvar(fvar3)) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar2),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg,
                                opand: target::Opand::reg(aux_reg),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar3),
                            };

                            let instr4 =
                                target::S::jump(target::Trg::reg(aux_reg_2));

                            let instr5 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4, instr5]
                        },
                    },

                    (
                        self::Loc::fvar(fvar),
                        self::Opand::int64(int64),
                        self::Trg::label(label),
                    ) => {
                        let (aux_reg, _) =
                            utils::Reg::current_auxiliary_registers();

                        let instr1 = target::S::set_reg_loc {
                            reg: aux_reg,
                            loc: target::Loc::fvar(fvar),
                        };

                        let instr2 = target::S::compare_reg_opand_jump_if {
                            reg: aux_reg,
                            opand: target::Opand::int64(int64),
                            relop,
                            label,
                        };

                        vec![instr1, instr2]
                    },
                    (
                        self::Loc::fvar(fvar),
                        self::Opand::int64(int64),
                        self::Trg::loc(loc),
                    ) => match loc {
                        self::Loc::reg(reg) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::int64(int64),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 = target::S::jump(target::Trg::reg(reg));

                            let instr4 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4]
                        },
                        self::Loc::fvar(fvar3) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::int64(int64),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar3),
                            };

                            let instr4 =
                                target::S::jump(target::Trg::reg(aux_reg_2));

                            let instr5 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4, instr5]
                        },
                    },
                    (
                        self::Loc::fvar(fvar),
                        self::Opand::loc(loc),
                        self::Trg::label(label),
                    ) => match loc {
                        self::Loc::reg(reg2) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(reg2),
                                relop,
                                label,
                            };

                            vec![instr1, instr2]
                        },
                        self::Loc::fvar(fvar2) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar2),
                            };

                            let instr3 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(aux_reg_2),
                                relop,
                                label,
                            };

                            vec![instr1, instr2, instr3]
                        },
                    },
                    (
                        self::Loc::fvar(fvar),
                        self::Opand::loc(loc),
                        self::Trg::loc(loc2),
                    ) => match (loc, loc2) {
                        (self::Loc::reg(reg2), self::Loc::reg(reg3)) => {
                            let (aux_reg, _) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(reg2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 =
                                target::S::jump(target::Trg::reg(reg3));

                            let instr4 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4]
                        },
                        (self::Loc::reg(reg2), self::Loc::fvar(fvar3)) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(reg2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr3 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar3),
                            };

                            let instr4 =
                                target::S::jump(target::Trg::reg(aux_reg_2));

                            let instr5 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4, instr5]
                        },
                        (self::Loc::fvar(fvar2), self::Loc::reg(reg3)) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar2),
                            };

                            let instr3 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(aux_reg_2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr4 =
                                target::S::jump(target::Trg::reg(reg3));

                            let instr5 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4, instr5]
                        },
                        (self::Loc::fvar(fvar2), self::Loc::fvar(fvar3)) => {
                            let (aux_reg, aux_reg_2) =
                                utils::Reg::current_auxiliary_registers();

                            let label = generate_neg_jump_label();

                            let instr1 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar),
                            };

                            let instr2 = target::S::set_reg_loc {
                                reg: aux_reg_2,
                                loc: target::Loc::fvar(fvar2),
                            };

                            let instr3 = target::S::compare_reg_opand_jump_if {
                                reg: aux_reg,
                                opand: target::Opand::reg(aux_reg_2),
                                relop: !relop,
                                label: label.clone(),
                            };

                            let instr4 = target::S::set_reg_loc {
                                reg: aux_reg,
                                loc: target::Loc::fvar(fvar3),
                            };

                            let instr5 =
                                target::S::jump(target::Trg::reg(aux_reg));

                            let instr6 = target::S::with_label {
                                label,
                                s: Box::new(target::S::nop),
                            };

                            vec![instr1, instr2, instr3, instr4, instr5, instr6]
                        },
                    },
                },

                self::S::nop => {
                    let instr = target::S::nop;

                    vec![instr]
                },
            }
        }

        let p = patch_p(p);

        target::ParenX64Fvars(p)
    }
}

fn generate_neg_jump_label() -> utils::Label {
    utils::Label::new_with_name("neg-jump")
}
