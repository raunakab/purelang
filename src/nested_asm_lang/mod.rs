pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::para_asm_lang as target;

pub struct NestedAsmLang {
    pub p: self::P,
}

/// FlattenBegins: NestedAsmLang -> ParaAsmLang
///
/// ### Purpose:
/// Flatten all nested begin expressions.
impl From<NestedAsmLang> for target::ParaAsmLang {
    fn from(NestedAsmLang { p }: NestedAsmLang) -> Self {
        fn flatten_p(p: self::P) -> target::P {
            match p {
                self::P::tail { tail } => {
                    let tail = Box::new(tail);
                    flatten_tail(tail)
                },
            }
        }

        fn flatten_tail(tail: Box<self::Tail>) -> target::P {
            match *tail {
                self::Tail::halt { triv } => {
                    let triv = flatten_triv(triv);
                    let halt = target::Halt { triv };
                    target::P::begin {
                        effects: vec![],
                        halt,
                    }
                },
                self::Tail::begin { effects, tail } => {
                    let mut effects = flatten_effects(effects);
                    let target::P::begin {
                        effects: tail_effects,
                        halt,
                    } = flatten_tail(tail);

                    effects.extend(tail_effects);

                    target::P::begin { effects, halt }
                },
            }
        }

        fn flatten_effects(effects: Vec<self::Effect>) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(flatten_effect)
                .flatten()
                .collect::<Vec<_>>()
        }

        fn flatten_effect(effect: self::Effect) -> Vec<target::Effect> {
            match effect {
                self::Effect::set_loc_triv { loc, triv } => {
                    let loc = flatten_loc(loc);
                    let triv = flatten_triv(triv);
                    let effect = target::Effect::set_loc_triv { loc, triv };

                    vec![effect]
                },
                self::Effect::set_loc_binop_triv { loc, binop, triv } => {
                    let loc = flatten_loc(loc);
                    let triv = flatten_triv(triv);
                    let effect =
                        target::Effect::set_loc_binop_triv { loc, binop, triv };

                    vec![effect]
                },
                self::Effect::begin { effects } => effects
                    .into_iter()
                    .map(flatten_effect)
                    .flatten()
                    .collect::<Vec<_>>(),
            }
        }

        fn flatten_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::loc { loc } => {
                    let loc = flatten_loc(loc);
                    target::Triv::loc { loc }
                },
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
            }
        }

        fn flatten_loc(loc: self::Loc) -> target::Loc {
            match loc {
                self::Loc::fvar { fvar } => target::Loc::fvar { fvar },
                self::Loc::reg { reg } => target::Loc::reg { reg },
            }
        }

        let p = flatten_p(p);
        Self { p }
    }
}
