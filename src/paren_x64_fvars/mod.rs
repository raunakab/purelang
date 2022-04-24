pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::cpsc411;
use crate::paren_x64 as target;

pub struct ParenX64Fvars {
    pub p: self::P,
}

impl ParenX64Fvars {
    /// ImplementFvars: ParenX64Fvars -> ParenX64
    ///
    /// ### Purpose:
    /// Compiles the Paren-x64-fvars v2 to Paren-x64 v2 by reifying fvars
    /// into displacement mode operands. The pass should use
    /// current-frame-base-pointer-register.
    pub fn implement_fvars(self) -> target::ParenX64 {
        let Self { p } = self;

        fn implement_p(p: self::P) -> target::P {
            match p {
                self::P::begin { ss } => {
                    let ss =
                        ss.into_iter().map(implement_s).collect::<Vec<_>>();
                    target::P::begin { ss }
                },
            }
        }

        fn implement_s(s: self::S) -> target::S {
            match s {
                self::S::set_fvar_int32 { fvar, int32 } => {
                    let addr = implement_fvar(fvar);
                    target::S::set_addr_int32 { addr, int32 }
                },
                self::S::set_fvar_trg { fvar, trg } => {
                    let addr = implement_fvar(fvar);
                    let trg = implement_trg(trg);

                    target::S::set_addr_trg { addr, trg }
                },
                self::S::set_reg_loc { reg, loc } => {
                    let loc = implement_loc(loc);
                    target::S::set_reg_loc { reg, loc }
                },
                self::S::set_reg_triv { reg, triv } => {
                    let triv = implement_triv(triv);
                    target::S::set_reg_triv { reg, triv }
                },
                self::S::set_reg_binop_reg_int32 { reg, binop, int32 } => {
                    target::S::set_reg_binop_reg_int32 { reg, binop, int32 }
                },
                self::S::set_reg_binop_reg_loc { reg, binop, loc } => {
                    let loc = implement_loc(loc);
                    target::S::set_reg_binop_reg_loc { reg, binop, loc }
                },
                self::S::with_label { label, s } => {
                    let s = implement_s(*s);
                    let s = Box::new(s);

                    target::S::with_label { label, s }
                },
                self::S::jump { trg } => {
                    let trg = implement_trg(trg);
                    target::S::jump_trg { trg }
                },
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                } => {
                    let opand = implement_opand(opand);
                    target::S::compare_reg_opand_jump_if {
                        reg,
                        opand,
                        relop,
                        label,
                    }
                },
                self::S::nop => target::S::nop,
            }
        }

        fn implement_fvar(
            cpsc411::Fvar { index }: cpsc411::Fvar,
        ) -> target::Addr {
            let fbp = cpsc411::Reg::current_frame_base_pointer();
            let disp_offset = index * 8;

            target::Addr { fbp, disp_offset }
        }

        fn implement_opand(opand: self::Opand) -> target::Opand {
            match opand {
                self::Opand::int64 { int64 } => target::Opand::int64 { int64 },
                self::Opand::reg { reg } => target::Opand::reg { reg },
            }
        }

        fn implement_loc(loc: self::Loc) -> target::Loc {
            match loc {
                self::Loc::reg { reg } => target::Loc::reg { reg },
                self::Loc::fvar { fvar } => {
                    let addr = implement_fvar(fvar);
                    target::Loc::addr { addr }
                },
            }
        }

        fn implement_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::trg { trg } => {
                    let trg = implement_trg(trg);
                    target::Triv::trg { trg }
                },
            }
        }

        fn implement_trg(trg: self::Trg) -> target::Trg {
            match trg {
                self::Trg::label { label } => target::Trg::label { label },
                self::Trg::reg { reg } => target::Trg::reg { reg },
            }
        }

        let p = implement_p(p);
        target::ParenX64 { p }
        // todo!()
    }
}
