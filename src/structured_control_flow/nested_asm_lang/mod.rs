pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::structured_control_flow::block_pred_lang as target;
use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct NestedAsmLang(pub self::P);

impl NestedAsmLang {
    /// ### Purpose:
    /// Optimize Nested-asm-lang v4 programs by analyzing and simplifying
    /// predicates.
    pub fn optimize_predicates(self) -> Self {
        self
    }

    /// ### Purpose:
    /// Compile the Nested-asm-lang v4 to Block-pred-lang v4, eliminating all
    /// nested expressions by generating fresh basic blocks and jumps.
    pub fn expose_basic_blocks(self) -> target::BlockPredLang {
        let Self(p) = self;

        fn expose_p(p: self::P) -> target::P {
            match p {
                self::P::module(tail) => {
                    let bs = vec![];
                    let (tail, mut bs) = expose_tail(tail, bs);
                    let label = utils::Label::new_with_name("main");
                    let b = target::B::define { label, tail };
                    bs.push(b);
                    target::P::module(bs)
                },
            }
        }

        fn expose_tail(
            tail: self::Tail,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match tail {
                self::Tail::halt(triv) => (target::Tail::halt(triv), bs),
                self::Tail::begin { effects, tail } => {
                    let (tail, bs) = expose_tail(*tail, bs);
                    expose_effects(effects, tail, bs)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let label_true = utils::Label::new_with_name("true");
                    let label_false = utils::Label::new_with_name("false");
                    let (tail1, bs) = expose_tail(*tail1, bs);
                    let (tail2, mut bs) = expose_tail(*tail2, bs);
                    let b1 = target::B::define {
                        label: label_true.clone(),
                        tail: tail1,
                    };
                    let b2 = target::B::define {
                        label: label_false.clone(),
                        tail: tail2,
                    };
                    bs.push(b1);
                    bs.push(b2);
                    expose_pred(pred, label_true, label_false, bs)
                },
            }
        }

        fn expose_effect(
            effect: self::Effect,
            tail: target::Tail,
            mut bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match effect {
                self::Effect::set { loc, triv } => {
                    let triv = target::Triv::opand(triv);
                    let effect = target::Effect::set { loc, triv };
                    let mut effects = vec![effect];
                    let tail =
                        make_begins!((effects, tail) => target::Tail::tail);
                    (tail, bs)
                },
                self::Effect::set_binop { loc, binop, triv } => {
                    let effect = target::Effect::set_binop {
                        loc,
                        binop,
                        opand: triv,
                    };
                    let mut effects = vec![effect];
                    let tail =
                        make_begins!((effects, tail) => target::Tail::tail);
                    (tail, bs)
                },
                self::Effect::begin(effects) => {
                    expose_effects(effects, tail, bs)
                },
                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let r#final = utils::Label::new_with_name("final");
                    let b = target::B::define {
                        label: r#final.clone(),
                        tail,
                    };
                    bs.push(b);
                    let tail = target::Tail::jump(target::Trg::label(r#final));
                    let (tail1, bs) = expose_effect(*effect1, tail.clone(), bs);
                    let (tail2, mut bs) = expose_effect(*effect2, tail, bs);
                    let label_true = utils::Label::new_with_name("true");
                    let label_false = utils::Label::new_with_name("false");
                    let b1 = target::B::define {
                        label: label_true.clone(),
                        tail: tail1,
                    };
                    let b2 = target::B::define {
                        label: label_false.clone(),
                        tail: tail2,
                    };
                    bs.push(b1);
                    bs.push(b2);
                    expose_pred(pred, label_true, label_false, bs)
                },
            }
        }

        fn expose_effects(
            effects: Vec<self::Effect>,
            tail: target::Tail,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            effects.into_iter().fold((tail, bs), |(tail, bs), effect| {
                expose_effect(effect, tail, bs)
            })
        }

        fn expose_pred(
            pred: self::Pred,
            label_true: utils::Label,
            label_false: utils::Label,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match pred {
                self::Pred::r#true => {
                    (target::Tail::jump(target::Trg::label(label_true)), bs)
                },
                self::Pred::r#false => {
                    (target::Tail::jump(target::Trg::label(label_false)), bs)
                },
                self::Pred::not(pred) => {
                    expose_pred(*pred, label_false, label_true, bs)
                },
                self::Pred::relop { relop, loc, triv } => {
                    let pred = target::Pred::relop {
                        relop,
                        loc,
                        opand: triv,
                    };
                    let tail = target::Tail::r#if {
                        pred,
                        trg1: target::Trg::label(label_true),
                        trg2: target::Trg::label(label_false),
                    };
                    (tail, bs)
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let l_true = utils::Label::new_with_name("true");
                    let l_false = utils::Label::new_with_name("false");
                    let (tail, bs) = expose_pred(
                        *pred1,
                        l_true.clone(),
                        l_false.clone(),
                        bs,
                    );
                    let (tail2, bs) = expose_pred(
                        *pred2,
                        label_true.clone(),
                        label_false.clone(),
                        bs,
                    );
                    let (tail3, mut bs) =
                        expose_pred(*pred3, label_true, label_false, bs);
                    let b2 = target::B::define {
                        label: l_true,
                        tail: tail2,
                    };
                    let b3 = target::B::define {
                        label: l_false,
                        tail: tail3,
                    };
                    bs.push(b2);
                    bs.push(b3);
                    (tail, bs)
                },

                self::Pred::begin { effects, pred } => {
                    let label_true = utils::Label::new_with_name("true");
                    let label_false = utils::Label::new_with_name("false");
                    let (tail, bs) =
                        expose_pred(*pred, label_true, label_false, bs);
                    expose_effects(effects, tail, bs)
                },
            }
        }

        let p = expose_p(p);
        target::BlockPredLang(p)
    }
}
