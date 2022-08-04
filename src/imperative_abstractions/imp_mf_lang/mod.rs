pub mod data;
#[cfg(test)]
mod tests;

use either::Either;

pub use self::data::*;
use crate::imperative_abstractions::imp_cmf_lang as target;
use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ImpMfLang(pub self::P);

impl ImpMfLang {
    /// ### Purpose:
    /// Compiles Imp-mf-lang v3 to Imp-cmf-lang v3, pushing set! under begin so
    /// that the right-hand-side of each set! is simple value-producing
    /// operation. This normalizes Imp-mf-lang v3 with respect to the
    /// equations.
    pub fn normalize_bind(self) -> target::ImpCmfLang {
        let Self(p) = self;

        fn normalize_p(p: self::P) -> target::P {
            match p {
                self::P::module(tail) => {
                    let tail = normalize_tail(tail);
                    target::P::module(tail)
                },
            }
        }

        fn normalize_tail(tail: self::Tail) -> target::Tail {
            match tail {
                self::Tail::value(value) => {
                    normalize_value(value, None).left().unwrap()
                },
                self::Tail::begin { effects, tail } => {
                    let effects = normalize_effects(effects);
                    let tail = normalize_tail(*tail);
                    let tail = Box::new(tail);
                    target::Tail::begin { effects, tail }
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let pred = normalize_pred(pred);
                    let tail1 = normalize_tail(*tail1);
                    let tail2 = normalize_tail(*tail2);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    target::Tail::r#if { pred, tail1, tail2 }
                },
            }
        }

        fn normalize_value(
            value: self::Value,
            aloc: Option<utils::Aloc>,
        ) -> Either<target::Tail, Vec<target::Effect>> {
            match aloc {
                Some(aloc) => {
                    let effects = match value {
                        self::Value::triv(triv) => {
                            let value = target::Value::triv(triv);
                            let effect =
                                target::Effect::set_aloc_value { aloc, value };
                            vec![effect]
                        },
                        self::Value::binop_triv_triv {
                            binop,
                            triv1,
                            triv2,
                        } => {
                            let value = target::Value::binop_triv_triv {
                                binop,
                                triv1,
                                triv2,
                            };
                            let effect =
                                target::Effect::set_aloc_value { aloc, value };
                            vec![effect]
                        },
                        self::Value::begin { effects, value } => {
                            let mut effects = normalize_effects(effects);
                            let effects_value =
                                normalize_value(*value, Some(aloc))
                                    .right()
                                    .unwrap();
                            effects.extend(effects_value);
                            effects
                        },
                        self::Value::r#if {
                            pred,
                            value1,
                            value2,
                        } => {
                            let pred = normalize_pred(pred);
                            let effects1 =
                                normalize_value(*value1, Some(aloc.clone()))
                                    .right()
                                    .unwrap();
                            let effects2 = normalize_value(*value2, Some(aloc))
                                .right()
                                .unwrap();
                            let effect1 = target::Effect::begin(effects1);
                            let effect2 = target::Effect::begin(effects2);
                            let effect1 = Box::new(effect1);
                            let effect2 = Box::new(effect2);
                            let effect = target::Effect::r#if {
                                pred,
                                effect1,
                                effect2,
                            };
                            vec![effect]
                        },
                    };

                    Either::Right(effects)
                },
                None => {
                    let tail = match value {
                        self::Value::triv(triv) => {
                            let value = target::Value::triv(triv);
                            target::Tail::value(value)
                        },
                        self::Value::binop_triv_triv {
                            binop,
                            triv1,
                            triv2,
                        } => {
                            let value = target::Value::binop_triv_triv {
                                binop,
                                triv1,
                                triv2,
                            };
                            target::Tail::value(value)
                        },
                        self::Value::begin { effects, value } => {
                            let effects = normalize_effects(effects);
                            let tail =
                                normalize_value(*value, None).left().unwrap();
                            let tail = Box::new(tail);
                            target::Tail::begin { effects, tail }
                        },
                        self::Value::r#if {
                            pred,
                            value1,
                            value2,
                        } => {
                            let pred = normalize_pred(pred);
                            let tail1 =
                                normalize_value(*value1, None).left().unwrap();
                            let tail2 =
                                normalize_value(*value2, None).left().unwrap();
                            let tail1 = Box::new(tail1);
                            let tail2 = Box::new(tail2);
                            target::Tail::r#if { pred, tail1, tail2 }
                        },
                    };

                    Either::Left(tail)
                },
            }
        }

        fn normalize_effect(effect: self::Effect) -> Vec<target::Effect> {
            match effect {
                self::Effect::set_aloc_value { aloc, value } => {
                    normalize_value(value, Some(aloc)).right().unwrap()
                },
                self::Effect::begin(effects) => normalize_effects(effects),
                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let pred = normalize_pred(pred);
                    let effects1 = normalize_effect(*effect1);
                    let effects2 = normalize_effect(*effect2);
                    let effect1 = target::Effect::begin(effects1);
                    let effect2 = target::Effect::begin(effects2);
                    let effect1 = Box::new(effect1);
                    let effect2 = Box::new(effect2);
                    let effect = target::Effect::r#if {
                        pred,
                        effect1,
                        effect2,
                    };
                    vec![effect]
                },
            }
        }

        fn normalize_pred(pred: self::Pred) -> target::Pred {
            match pred {
                self::Pred::relop {
                    relop,
                    triv1,
                    triv2,
                } => target::Pred::relop {
                    relop,
                    triv1,
                    triv2,
                },
                self::Pred::r#true => target::Pred::r#true,
                self::Pred::r#false => target::Pred::r#false,
                self::Pred::not(pred) => {
                    let pred = normalize_pred(*pred);
                    let pred = Box::new(pred);
                    target::Pred::not(pred)
                },
                self::Pred::begin { effects, pred } => {
                    let effects = normalize_effects(effects);
                    let pred = normalize_pred(*pred);
                    let pred = Box::new(pred);
                    target::Pred::begin { effects, pred }
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let pred1 = normalize_pred(*pred1);
                    let pred2 = normalize_pred(*pred2);
                    let pred3 = normalize_pred(*pred3);
                    let pred1 = Box::new(pred1);
                    let pred2 = Box::new(pred2);
                    let pred3 = Box::new(pred3);
                    target::Pred::r#if {
                        pred1,
                        pred2,
                        pred3,
                    }
                },
            }
        }

        fn normalize_effects(
            effects: Vec<self::Effect>,
        ) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(normalize_effect)
                .flatten()
                .collect()
        }

        let p = normalize_p(p);
        target::ImpCmfLang(p)
    }
}
