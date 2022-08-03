pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::utils;
use crate::x64::paren_x64 as target;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ParenX64Fvars(pub self::P);

impl ParenX64Fvars {
    /// ### Purpose:
    /// Compiles the Paren-x64-fvars v2 to Paren-x64 v2 by reifying fvars
    /// into displacement mode operands. The pass should use
    /// current-frame-base-pointer-register.
    pub fn implement_fvars(self) -> target::ParenX64 {
        let Self(p) = self;

        fn implement_p(p: self::P) -> target::P {
            match p {
                self::P::begin(ss) => {
                    let ss =
                        ss.into_iter().map(implement_s).collect::<Vec<_>>();

                    target::P::begin(ss)
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

                    target::S::set_addr_trg { addr, trg }
                },
                self::S::set_reg_loc { reg, loc } => {
                    let loc = implement_loc(loc);

                    target::S::set_reg_loc { reg, loc }
                },
                self::S::set_reg_triv { reg, triv } => {
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
                self::S::jump(trg) => target::S::jump_trg(trg),
                self::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                } => target::S::compare_reg_opand_jump_if {
                    reg,
                    opand,
                    relop,
                    label,
                },
                self::S::nop => target::S::nop,
            }
        }

        fn implement_fvar(
            utils::Fvar(index): utils::Fvar,
        ) -> utils::Addr {
            let fbp = utils::Reg::current_frame_base_pointer();

            let disp_offset = index * 8;

            utils::Addr { fbp, disp_offset }
        }

        fn implement_loc(loc: self::Loc) -> target::Loc {
            match loc {
                self::Loc::reg(reg) => target::Loc::reg(reg),
                self::Loc::fvar(fvar) => {
                    let addr = implement_fvar(fvar);

                    target::Loc::addr(addr)
                },
            }
        }

        let p = implement_p(p);

        target::ParenX64(p)
    }
}
