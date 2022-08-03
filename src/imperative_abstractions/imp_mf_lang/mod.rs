pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::utils;
use crate::imperative_abstractions::imp_cmf_lang as target;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ImpMfLang {
    pub p: self::P,
}

impl ImpMfLang {
    /// ### Purpose:
    /// Compiles Imp-mf-lang v3 to Imp-cmf-lang v3, pushing set! under begin so
    /// that the right-hand-side of each set! is simple value-producing
    /// operation. This normalizes Imp-mf-lang v3 with respect to the
    /// equations.
    pub fn normalize_bind(self) -> target::ImpCmfLang {
        let Self { p } = self;

        fn normalize_p(p: self::P) -> target::P {
            match p {
                self::P::module { tail } => {
                    let tail = Box::new(tail);
                    let tail = normalize_tail(tail);

                    target::P::module(tail)
                },
            }
        }

        fn normalize_tail(tail: Box<self::Tail>) -> target::Tail {
            match *tail {
                self::Tail::value { value } => {
                    let value = Box::new(value);
                    normalize_value_in_tail(value)
                },
                self::Tail::begin { effects, tail } => {
                    let effects = normalize_effects(effects);
                    let tail = normalize_tail(tail);
                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
            }
        }

        fn normalize_value_in_tail(value: Box<self::Value>) -> target::Tail {
            match *value {
                self::Value::triv { triv } => {
                    let triv = normalize_triv(triv);

                    let value = target::Value::triv(triv);

                    target::Tail::value(value)
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let triv1 = normalize_triv(triv1);

                    let triv2 = normalize_triv(triv2);

                    let value = target::Value::binop_triv_triv {
                        binop,
                        triv1,
                        triv2,
                    };

                    target::Tail::value(value)
                },
                self::Value::begin { effects, value } => {
                    let effects = normalize_effects(effects);
                    let tail = normalize_value_in_tail(value);
                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
            }
        }

        fn normalize_value(
            value: Box<self::Value>,
            aloc: utils::Aloc,
        ) -> Vec<target::Effect> {
            match *value {
                self::Value::triv { triv } => {
                    let triv = normalize_triv(triv);

                    let value = target::Value::triv(triv);

                    let effect = target::Effect::set_aloc_value { aloc, value };

                    vec![effect]
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let triv1 = normalize_triv(triv1);

                    let triv2 = normalize_triv(triv2);

                    let value = target::Value::binop_triv_triv {
                        binop,
                        triv1,
                        triv2,
                    };

                    let effect = target::Effect::set_aloc_value { aloc, value };

                    vec![effect]
                },
                self::Value::begin { effects, value } => {
                    let mut effects = normalize_effects(effects);

                    let value = normalize_value(value, aloc);

                    effects.extend(value);

                    effects
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

        fn normalize_effect(effect: self::Effect) -> Vec<target::Effect> {
            match effect {
                self::Effect::set_aloc_value { aloc, value } => {
                    let value = Box::new(value);

                    normalize_value(value, aloc)
                },
                self::Effect::begin { effects } => normalize_effects(effects),
            }
        }

        fn normalize_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64(int64),
                self::Triv::aloc { aloc } => target::Triv::aloc(aloc),
            }
        }

        let p = normalize_p(p);

        target::ImpCmfLang(p)
    }
}
