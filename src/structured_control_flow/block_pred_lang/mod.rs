mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::structured_control_flow::block_asm_lang as target;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct BlockPredLang(pub self::P);

impl BlockPredLang {
    /// ### Purpose:
    /// Compile the Block-pred-lang v4 to Block-asm-lang v4 by manipulating the
    /// branches of if statements to resolve branches.
    pub fn resolve_predicates(self) -> target::BlockAsmLang {
        let Self(p) = self;

        fn resolve_p(p: self::P) -> target::P {
            match p {
                self::P::module(bs) => {
                    let bs = resolve_bs(bs);

                    target::P::module(bs)
                },
            }
        }

        fn resolve_bs(bs: Vec<self::B>) -> Vec<target::B> {
            bs.into_iter().map(resolve_b).collect()
        }

        fn resolve_b(b: self::B) -> target::B {
            match b {
                self::B::define { label, tail } => {
                    let tail = resolve_tail(tail);

                    target::B::define_label_tail { label, tail }
                },
            }
        }

        fn resolve_tail(tail: self::Tail) -> target::Tail {
            match tail {
                self::Tail::halt(opand) => target::Tail::halt(opand),
                self::Tail::jump(trg) => target::Tail::jump(trg),
                self::Tail::begin { effects, tail } => {
                    let tail = resolve_tail(*tail);

                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
                self::Tail::r#if { pred, trg1, trg2 } => match pred {
                    self::Pred::relop { relop, loc, opand } => {
                        target::Tail::r#if {
                            relop,
                            loc,
                            opand,
                            trg1,
                            trg2,
                        }
                    },
                    self::Pred::r#true => target::Tail::jump(trg1),
                    self::Pred::r#false => target::Tail::jump(trg2),
                    self::Pred::not(pred) => {
                        let tail = self::Tail::r#if {
                            pred: *pred,
                            trg1: trg2,
                            trg2: trg1,
                        };

                        resolve_tail(tail)
                    },
                },
            }
        }

        let p = resolve_p(p);

        target::BlockAsmLang(p)
    }
}
