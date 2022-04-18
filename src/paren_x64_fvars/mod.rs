pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::cpsc411;
use crate::paren_x64 as target;
use crate::paren_x64::ParenX64;

pub struct ParenX64Fvars {
    pub p: self::P,
}

/// ImplementFvars: ParenX64Fvars -> ParenX64
///
/// ### Purpose
/// Compiles the Paren-x64-fvars v2 to Paren-x64 v2 by reifying fvars
/// into displacement mode operands. The pass should use
/// current-frame-base-pointer-register.
impl From<ParenX64Fvars> for ParenX64 {
    fn from(paren_x64_fvars: ParenX64Fvars) -> Self {
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

                self::S::set_fvar_reg { fvar, reg } => {
                    let addr = implement_fvar(fvar);
                    target::S::set_addr_reg { addr, reg }
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
            }
        }

        fn implement_fvar(
            cpsc411::Fvar { index }: cpsc411::Fvar,
        ) -> target::Addr {
            let fbp = cpsc411::Reg::current_frame_base_pointer();
            let disp_offset = index * 8;

            target::Addr { fbp, disp_offset }
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
                self::Triv::reg { reg } => target::Triv::reg { reg },
            }
        }

        let p = implement_p(paren_x64_fvars.p);
        Self { p }
    }
}
