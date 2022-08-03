pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::register_allocation::asm_pred_lang as target;
use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ImpCmfLang(pub self::P);

impl ImpCmfLang {
    /// ### Purpose:
    /// Compiles Imp-cmf-lang v3 to Asm-lang v2, selecting appropriate sequences
    /// of abstract assembly instructions to implement the operations of the
    /// source language.
    pub fn select_instructions(self) -> target::AsmLang {
        let Self(p) = self;

        fn select_p(p: self::P) -> target::P {
            match p {
                self::P::module(tail) => {
                    let (mut effects, tail) = select_tail(tail);
                    let tail = make_begins!((effects, tail) => target::Tail::tail);
                    target::P::module {
                        info: utils::Info::default(),
                        tail,
                    }
                },
            }
        }

        fn select_tail(
            tail: self::Tail,
        ) -> (Vec<target::Effect>, target::Tail) {
            match tail {
                self::Tail::value(value) => match value {
                    self::Value::triv(triv) => {
                        (Vec::with_capacity(0), target::Tail::halt(triv))
                    },
                    self::Value::binop_triv_triv {
                        binop,
                        triv1,
                        triv2,
                    } => {
                        let aloc = utils::Aloc::fresh();
                        let effect1 = target::Effect::set_aloc_triv {
                            aloc: aloc.clone(),
                            triv: triv1,
                        };
                        let effect2 = target::Effect::set_aloc_binop_aloc_triv {
                            aloc: aloc.clone(),
                            binop,
                            triv: triv2,
                        };
                        let effects = vec![effect1, effect2];
                        let triv = target::Triv::aloc(aloc);
                        let tail = target::Tail::halt(triv);
                        (effects, tail)
                    },
                },
                self::Tail::begin { effects, tail } => {
                    let mut effects = select_effects(effects);
                    let (tail_effects, tail) = select_tail(*tail);
                    effects.extend(tail_effects);
                    (effects, tail)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let (effects, pred) = select_pred(pred);
                    let (mut effects1, tail1) = select_tail(*tail1);
                    let (mut effects2, tail2) = select_tail(*tail2);
                    let tail1 = make_begins!((effects1, tail1) => target::Tail::tail);
                    let tail2 = make_begins!((effects2, tail2) => target::Tail::tail);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    let tail = target::Tail::r#if { pred, tail1, tail2 };
                    (effects, tail)
                },
            }
        }

        fn select_pred(
            pred: self::Pred,
        ) -> (Vec<target::Effect>, target::Pred) {
            match pred {
                self::Pred::relop { relop, triv1, triv2 } => {
                    let aloc = utils::Aloc::fresh();
                    let effect = target::Effect::set_aloc_triv { aloc: aloc.clone(), triv: triv1 };
                    let effects = vec![effect];
                    let pred = target::Pred::relop { relop, aloc, triv: triv2 };
                    (effects, pred)
                },
                self::Pred::r#true => (Vec::with_capacity(0), target::Pred::r#true),
                self::Pred::r#false => (Vec::with_capacity(0), target::Pred::r#false),
                self::Pred::not(pred) => select_pred(*pred),
                self::Pred::begin { effects, pred } => {
                    let mut effects = select_effects(effects);
                    let (pred_effects, pred) = select_pred(*pred);
                    effects.extend(pred_effects);
                    (effects, pred)
                },
                self::Pred::r#if { pred1, pred2, pred3 } => {
                    let (effects, pred1) = select_pred(*pred1);
                    let (mut effects2, pred2) = select_pred(*pred2);
                    let (mut effects3, pred3) = select_pred(*pred3);
                    let pred2 = make_begins!((effects2, pred2) => target::Pred::pred);
                    let pred3 = make_begins!((effects3, pred3) => target::Pred::pred);
                    let pred1 = Box::new(pred1);
                    let pred2 = Box::new(pred2);
                    let pred3 = Box::new(pred3);
                    (effects, target::Pred::r#if { pred1, pred2, pred3 })
                },
            }
        }

        fn select_effects(effects: Vec<self::Effect>) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(select_effect)
                .flatten()
                .collect::<Vec<_>>()
        }

        fn select_effect(effect: self::Effect) -> Vec<target::Effect> {
            match effect {
                self::Effect::set_aloc_value { aloc, value } => select_value(value, aloc),
                self::Effect::begin(effects) => select_effects(effects),
                self::Effect::r#if { pred, effect1, effect2 } => {
                    let (mut effects, pred) = select_pred(pred);
                    let effects1 = select_effect(*effect1);
                    let effects2 = select_effect(*effect2);
                    let effect1 = target::Effect::begin(effects1);
                    let effect2 = target::Effect::begin(effects2);
                    let effect1 = Box::new(effect1);
                    let effect2 = Box::new(effect2);
                    let effect = target::Effect::r#if { pred, effect1, effect2 };
                    effects.push(effect);
                    effects
                },
            }
        }

        fn select_value(
            value: self::Value,
            aloc: utils::Aloc,
        ) -> Vec<target::Effect> {
            match value {
                self::Value::triv(triv) => {
                    let effect = target::Effect::set_aloc_triv { aloc, triv };
                    vec![effect]
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let effect1 = target::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: triv1,
                    };
                    let effect2 = target::Effect::set_aloc_binop_aloc_triv {
                        aloc,
                        binop,
                        triv: triv2,
                    };
                    vec![effect1, effect2]
                },
            }
        }

        let p = select_p(p);
        target::AsmLang(p)
    }
}
