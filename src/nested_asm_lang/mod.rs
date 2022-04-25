pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::block_pred_lang as target;

pub struct NestedAsmLang {
    pub p: self::P,
}

impl NestedAsmLang {
    /// ExposeBasicBlocks: NestedAsmLang ->BlockPredLang
    ///
    /// ### Purpose:
    /// Compile the Nested-asm-lang v4 to Block-pred-lang v4, eliminating all
    /// nested expressions by generating fresh basic blocks and jumps.
    pub fn expose_basic_blocks(self) -> target::BlockPredLang {
        let Self { p } = self;

        fn expose_p(p: self::P) -> target::P {
            match p {
                self::P::module { tail } => {
                    let bs = expose_tail(tail);
                    target::P::module { bs }
                },
            }
        }

        fn expose_tail(tail: self::Tail) -> Vec<target::B> {
            match tail {
                self::Tail::halt { .. } => vec![],
                _ => todo!(),
            }
        }

        let p = expose_p(p);
        target::BlockPredLang { p }
    }

    /// OptimizePredicates: NestedAsmLang -> NestedAsmLang
    ///
    /// ### Purpose:
    /// Optimize Nested-asm-lang v4 programs by analyzing and simplifying
    /// predicates.
    pub fn optimize_predicates(self) -> Self {
        self
    }

    // /// FlattenBegins: NestedAsmLang -> ParaAsmLang
    // ///
    // /// ### Purpose:
    // /// Flatten all nested begin expressions.
    // pub fn flatten_begins(self) -> target::ParaAsmLang {
    //     let Self { p } = self;
    //     fn flatten_p(p: self::P) -> target::P {
    //         match p {
    //             self::P::tail { tail } => {
    //                 todo!()
    //                 // let tail = Box::new(tail);
    //                 // flatten_tail(tail)
    //             },
    //         }
    //     }
    //     fn flatten_tail(tail: Box<self::Tail>) -> target::P {
    //         match *tail {
    //             self::Tail::begin { effects, tail } => {
    //                 // let mut effects = flatten_effects(effects);
    //                 // let target::P::begin {
    //                 //     effects: tail_effects,
    //                 //     halt,
    //                 // } = flatten_tail(tail);
    //                 // effects.extend(tail_effects);
    //                 // target::P::begin { effects, halt }
    //                 todo!()
    //             },
    //             self::Tail::halt { triv } => {
    //                 // let triv = flatten_triv(triv);
    //                 // let halt = target::Halt { triv };
    //                 // target::P::begin {
    //                 //     effects: vec![],
    //                 //     halt,
    //                 // }
    //                 todo!()
    //             },
    //         }
    //     }
    //     fn flatten_effects(effects: Vec<self::Effect>) -> Vec<target::S> {
    //         effects
    //             .into_iter()
    //             .map(flatten_effect)
    //             .flatten()
    //             .collect::<Vec<_>>()
    //     }
    //     fn flatten_effect(effect: self::Effect) -> Vec<target::S> {
    //         match effect {
    //             self::Effect::set_loc_triv { loc, triv } => {
    //                 let loc = flatten_loc(loc);
    //                 let triv = flatten_triv(triv);
    //                 let effect = target::S::set_loc_triv { loc, triv };
    //                 vec![effect]
    //             },
    //             self::Effect::set_loc_binop_triv { loc, binop, triv } => {
    //                 // let loc = flatten_loc(loc);
    //                 // let triv = flatten_triv(triv);
    //                 // let effect =
    //                 //     target::S::set_loc_binop_opand { loc, binop, triv
    // };                 // vec![effect]
    //                 todo!()
    //             },
    //             self::Effect::begin { effects } => effects
    //                 .into_iter()
    //                 .map(flatten_effect)
    //                 .flatten()
    //                 .collect::<Vec<_>>(),
    //         }
    //     }
    //     fn flatten_triv(triv: self::Triv) -> target::Triv {
    //         match triv {
    //             self::Triv::loc { loc } => {
    //                 // let loc = flatten_loc(loc);
    //                 // target::Triv::loc { loc }
    //                 todo!()
    //             },
    //             self::Triv::int64 { int64 } => {
    //                 // target::Triv::int64 { int64 }
    //                 todo!()
    //             },
    //         }
    //     }
    //     fn flatten_loc(loc: self::Loc) -> target::Loc {
    //         match loc {
    //             self::Loc::fvar { fvar } => target::Loc::fvar { fvar },
    //             self::Loc::reg { reg } => target::Loc::reg { reg },
    //         }
    //     }
    //     let p = flatten_p(p);
    //     target::ParaAsmLang { p }
    // }
}
