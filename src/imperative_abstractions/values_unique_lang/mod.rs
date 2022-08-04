pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::imperative_abstractions::imp_mf_lang as target;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ValuesUniqueLang(pub self::P);

impl ValuesUniqueLang {
    /// OptimizeLetBindings: ValuesUniqueLang -> ValuesUniqueLang
    ///
    /// ### Purpose:
    /// Optimizes let bindings by reordering them to minimize or maximize some
    /// metric.
    pub fn optimize_let_bindings(self) -> Self {
        self
    }

    /// SequentializeLet: ValuesUniqueLang -> ImpMfLang
    ///
    /// ### Purpose:
    /// Compiles Values-unique-lang v3 to Imp-mf-lang v3 by picking a particular
    /// order to implement let expressions using set!.
    pub fn sequentialize_let(self) -> target::ImpMfLang {
        let Self(p) = self;

        fn seq_p(p: self::P) -> target::P {
            match p {
                self::P::module(tail) => {
                    let tail = seq_tail(tail);

                    target::P::module(tail)
                },
            }
        }

        fn seq_tail(tail: self::Tail) -> target::Tail {
            match tail {
                self::Tail::value(value) => {
                    let value = seq_value(value);
                    target::Tail::value(value)
                },
                self::Tail::r#let { bindings, tail } => {
                    let effects = seq_bindings(bindings);
                    let tail = seq_tail(*tail);
                    let tail = Box::new(tail);
                    target::Tail::begin { effects, tail }
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let pred = seq_pred(pred);
                    let tail1 = seq_tail(*tail1);
                    let tail2 = seq_tail(*tail2);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    target::Tail::r#if { pred, tail1, tail2 }
                },
            }
        }

        fn seq_pred(pred: self::Pred) -> target::Pred {
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
                self::Pred::not(pred) => seq_pred(*pred),
                self::Pred::r#let { bindings, pred } => {
                    let effects = seq_bindings(bindings);
                    let pred = seq_pred(*pred);
                    let pred = Box::new(pred);
                    target::Pred::begin { effects, pred }
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let pred1 = seq_pred(*pred1);
                    let pred2 = seq_pred(*pred2);
                    let pred3 = seq_pred(*pred3);
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

        fn seq_value(value: self::Value) -> target::Value {
            match value {
                self::Value::triv(triv) => target::Value::triv(triv),
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => target::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                },
                self::Value::r#let { bindings, value } => {
                    let effects = seq_bindings(bindings);
                    let value = seq_value(*value);
                    let value = Box::new(value);
                    target::Value::begin { effects, value }
                },
                self::Value::r#if {
                    pred,
                    value1,
                    value2,
                } => {
                    let pred = seq_pred(pred);
                    let value1 = seq_value(*value1);
                    let value2 = seq_value(*value2);
                    let value1 = Box::new(value1);
                    let value2 = Box::new(value2);
                    target::Value::r#if {
                        pred,
                        value1,
                        value2,
                    }
                },
            }
        }

        fn seq_bindings(bindings: self::Bindings) -> Vec<target::Effect> {
            bindings
                .into_iter()
                .map(|(aloc, value)| {
                    let value = seq_value(value);

                    target::Effect::set_aloc_value { aloc, value }
                })
                .collect::<Vec<_>>()
        }

        let p = seq_p(p);
        target::ImpMfLang(p)
    }
}
