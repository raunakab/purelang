pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::cpsc411;
use crate::paren_x64_fvars as target;

pub struct ParaAsmLang {
    pub p: self::P,
}

/// PatchInstructions: ParaAsmLang -> ParenX64Fvars
///
/// ### Purpose:
/// Compiles Para-asm-lang v2 to Paren-x64-fvars v2 by patching instructions
/// that have no x64 analogue into a sequence of instructions. The
/// implementation should use auxiliary registers from
/// current-patch-instructions-registers when generating instruction sequences,
/// and current-return-value-register for compiling halt.
impl From<ParaAsmLang> for target::ParenX64Fvars {
    fn from(ParaAsmLang { p }: ParaAsmLang) -> Self {
        fn patch_p(p: self::P) -> target::P {
            match p {
                self::P::begin { effects, halt } => {
                    let mut ss = effects
                        .into_iter()
                        .map(patch_s)
                        .flatten()
                        .collect::<Vec<_>>();

                    let halt_s = patch_halt(halt);
                    ss.push(halt_s);

                    target::P::begin { ss }
                },
            }
        }

        fn patch_s(effect: self::Effect) -> Vec<target::S> {
            match effect {
                self::Effect::set_loc_triv { loc, triv } => match (loc, triv) {
                    // reg <- loc
                    (self::Loc::reg { reg }, self::Triv::loc { loc }) => {
                        let loc = patch_loc(loc);
                        let instr = target::S::set_reg_loc { reg, loc };

                        vec![instr]
                    },

                    // reg <- int64
                    (self::Loc::reg { reg }, self::Triv::int64 { int64 }) => {
                        let triv = target::Triv::int64 { int64 };
                        let instr = target::S::set_reg_triv { reg, triv };

                        vec![instr]
                    },

                    (self::Loc::fvar { fvar }, self::Triv::loc { loc }) => {
                        match loc {
                            // fvar <- reg
                            self::Loc::reg { reg } => {
                                let instr =
                                    target::S::set_fvar_reg { fvar, reg };
                                vec![instr]
                            },

                            // r10 <- fvar2
                            // fvar <- r10
                            self::Loc::fvar { fvar: fvar2 } => {
                                #[allow(bindings_with_variant_name)]
                                let (r10, _) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                let loc = target::Loc::fvar { fvar: fvar2 };
                                let instr_1 =
                                    target::S::set_reg_loc { reg: r10, loc };
                                let instr_2 =
                                    target::S::set_fvar_reg { fvar, reg: r10 };

                                vec![instr_1, instr_2]
                            },
                        }
                    },

                    (self::Loc::fvar { fvar }, self::Triv::int64 { int64 }) => {
                        i32::try_from(int64).ok().map_or_else(
                            // r10 <- int64
                            // fvar <- r10
                            || {
                                #[allow(bindings_with_variant_name)]
                                let (r10, _) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                let triv = target::Triv::int64 { int64 };
                                let instr_1 =
                                    target::S::set_reg_triv { reg: r10, triv };
                                let instr_2 =
                                    target::S::set_fvar_reg { fvar, reg: r10 };

                                vec![instr_1, instr_2]
                            },
                            // fvar <- int32
                            |int32| {
                                let instr =
                                    target::S::set_fvar_int32 { fvar, int32 };
                                vec![instr]
                            },
                        )
                    },
                },
                self::Effect::set_loc_binop_triv { loc, binop, triv } => {
                    match (loc, triv) {
                        // reg <- reg + loc
                        (self::Loc::reg { reg }, self::Triv::loc { loc }) => {
                            let loc = patch_loc(loc);
                            let instr = target::S::set_reg_binop_reg_loc {
                                reg,
                                binop,
                                loc,
                            };

                            vec![instr]
                        },

                        (
                            self::Loc::reg { reg },
                            self::Triv::int64 { int64 },
                        ) => i32::try_from(int64).ok().map_or_else(
                            // r10 <- int64
                            // reg <- reg + r10
                            || {
                                #[allow(bindings_with_variant_name)]
                                let (r10, _) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                let triv = target::Triv::int64 { int64 };
                                let instr_1 =
                                    target::S::set_reg_triv { reg: r10, triv };

                                let loc = target::Loc::reg { reg: r10 };
                                let instr_2 =
                                    target::S::set_reg_binop_reg_loc {
                                        reg,
                                        binop,
                                        loc,
                                    };

                                vec![instr_1, instr_2]
                            },
                            // reg <- reg + int32
                            |int32| {
                                let instr =
                                    target::S::set_reg_binop_reg_int32 {
                                        reg,
                                        binop,
                                        int32,
                                    };
                                vec![instr]
                            },
                        ),
                        (self::Loc::fvar { fvar }, self::Triv::loc { loc }) => {
                            match loc {
                                // r10 <- fvar
                                // r10 <- reg + r10
                                // fvar <- r10
                                self::Loc::reg { reg } => {
                                    #[allow(bindings_with_variant_name)]
                                let (r10, _) = cpsc411::Reg::current_auxiliary_registers();

                                    let loc = target::Loc::fvar { fvar };
                                    let instr_1 = target::S::set_reg_loc {
                                        reg: r10,
                                        loc,
                                    };

                                    let loc = target::Loc::reg { reg };
                                    let instr_2 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg: r10,
                                            binop,
                                            loc,
                                        };

                                    let instr_3 = target::S::set_fvar_reg {
                                        fvar,
                                        reg: r10,
                                    };

                                    vec![instr_1, instr_2, instr_3]
                                },

                                // r10 <- fvar
                                // r11 <- fvar2
                                // r10 <- r10 + r11
                                // fvar <- r10
                                self::Loc::fvar { fvar: fvar2 } => {
                                    #[allow(bindings_with_variant_name)]
                                let (r10, r11) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                    let loc = target::Loc::fvar { fvar };
                                    let instr_1 = target::S::set_reg_loc {
                                        reg: r10,
                                        loc,
                                    };

                                    let loc = target::Loc::fvar { fvar: fvar2 };
                                    let instr_2 = target::S::set_reg_loc {
                                        reg: r11,
                                        loc,
                                    };

                                    let loc = target::Loc::reg { reg: r11 };
                                    let instr_3 =
                                        target::S::set_reg_binop_reg_loc {
                                            reg: r10,
                                            binop,
                                            loc,
                                        };

                                    let instr_4 = target::S::set_fvar_reg {
                                        fvar,
                                        reg: r10,
                                    };

                                    vec![instr_1, instr_2, instr_3, instr_4]
                                },
                            }
                        },
                        (
                            self::Loc::fvar { fvar },
                            self::Triv::int64 { int64 },
                        ) => i32::try_from(int64).ok().map_or_else(
                            // r10 <- fvar
                            // r11 <- int64
                            // r10 <- r10 + r11
                            // fvar <- r10
                            || {
                                #[allow(bindings_with_variant_name)]
                                let (r10, r11) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                let loc = target::Loc::fvar { fvar };
                                let instr_1 =
                                    target::S::set_reg_loc { reg: r10, loc };

                                let triv = target::Triv::int64 { int64 };
                                let instr_2 =
                                    target::S::set_reg_triv { reg: r11, triv };

                                let loc = target::Loc::reg { reg: r11 };
                                let instr_3 =
                                    target::S::set_reg_binop_reg_loc {
                                        reg: r10,
                                        binop,
                                        loc,
                                    };

                                let instr_4 =
                                    target::S::set_fvar_reg { fvar, reg: r10 };

                                vec![instr_1, instr_2, instr_3, instr_4]
                            },
                            // r10 <- fvar
                            // r10 <- r10 + int32
                            // fvar <- r10
                            |int32| {
                                #[allow(bindings_with_variant_name)]
                                let (r10, _) =
                                    cpsc411::Reg::current_auxiliary_registers();

                                let loc = target::Loc::fvar { fvar };
                                let instr_1 =
                                    target::S::set_reg_loc { reg: r10, loc };
                                let instr_2 =
                                    target::S::set_reg_binop_reg_int32 {
                                        reg: r10,
                                        binop,
                                        int32,
                                    };
                                let instr_3 =
                                    target::S::set_fvar_reg { fvar, reg: r10 };

                                vec![instr_1, instr_2, instr_3]
                            },
                        ),
                    }
                },
            }
        }

        fn patch_halt(Halt { triv }: self::Halt) -> target::S {
            match triv {
                self::Triv::loc { loc } => {
                    let loc = patch_loc(loc);
                    let reg = cpsc411::Reg::rax;

                    target::S::set_reg_loc { reg, loc }
                },
                self::Triv::int64 { int64 } => {
                    let reg = cpsc411::Reg::rax;
                    let triv = target::Triv::int64 { int64 };

                    target::S::set_reg_triv { reg, triv }
                },
            }
        }

        fn patch_loc(loc: self::Loc) -> target::Loc {
            match loc {
                self::Loc::reg { reg } => target::Loc::reg { reg },
                self::Loc::fvar { fvar } => target::Loc::fvar { fvar },
            }
        }

        let p = patch_p(p);
        Self { p }
    }
}
