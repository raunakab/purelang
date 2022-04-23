pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use self::data::*;
use crate::cpsc411;
use crate::imp_mf_lang as target;

#[derive(Debug, PartialEq, Eq)]
pub struct ValuesUniqueLang {
    pub p: self::P,
}

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
        let Self { p } = self;

        fn seq_p(p: self::P) -> target::P {
            match p {
                self::P::module { tail } => {
                    let tail = Box::new(tail);
                    let tail = seq_tail(tail);

                    target::P::module { tail }
                },
            }
        }

        fn seq_tail(tail: Box<self::Tail>) -> target::Tail {
            match *tail {
                self::Tail::value { value } => {
                    let value = Box::new(value);
                    let value = seq_value(value);

                    target::Tail::value { value }
                },
                self::Tail::r#let { bindings, tail } => {
                    let effects = seq_bindings(bindings);
                    let tail = seq_tail(tail);
                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
            }
        }

        fn seq_value(value: Box<self::Value>) -> target::Value {
            match *value {
                self::Value::triv { triv } => {
                    let triv = seq_triv(triv);
                    target::Value::triv { triv }
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let triv1 = seq_triv(triv1);
                    let triv2 = seq_triv(triv2);

                    target::Value::binop_triv_triv {
                        binop,
                        triv1,
                        triv2,
                    }
                },
                self::Value::r#let { bindings, value } => {
                    let effects = seq_bindings(bindings);
                    let value = seq_value(value);
                    let value = Box::new(value);

                    target::Value::begin { effects, value }
                },
            }
        }

        fn seq_bindings(
            bindings: HashMap<cpsc411::Aloc, self::Value>,
        ) -> Vec<target::Effect> {
            bindings
                .into_iter()
                .map(|(aloc, value)| {
                    let value = Box::new(value);
                    let value = seq_value(value);

                    target::Effect::set_aloc_value { aloc, value }
                })
                .collect::<Vec<_>>()
        }

        fn seq_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::aloc { aloc } => target::Triv::aloc { aloc },
            }
        }

        let p = seq_p(p);
        target::ImpMfLang { p }
    }
}
