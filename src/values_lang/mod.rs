pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use self::data::*;
use crate::cpsc411;
use crate::values_unique_lang as target;

pub struct ValuesLang {
    pub p: self::P,
}

impl ValuesLang {
    /// Uniquify: ValuesLang -> ValuesUniqueLang
    ///
    /// ### Purpose:
    /// Compiles Values-lang v3 to Values-unique-lang v3 by resolving all
    /// lexical identifiers to abstract locations.
    pub fn uniquify(self) -> target::ValuesUniqueLang {
        let Self { p } = self;

        fn uniquify_p(p: self::P) -> target::P {
            match p {
                self::P::module { tail } => {
                    let tail = Box::new(tail);
                    let tail = uniquify_tail(tail);
                    target::P::module { tail }
                },
            }
        }

        fn uniquify_tail(tail: Box<self::Tail>) -> target::Tail {
            match *tail {
                self::Tail::value { value } => {
                    let value = Box::new(value);
                    let value = uniquify_value(value);
                    target::Tail::value { value }
                },
                self::Tail::r#let { bindings, tail } => {
                    let bindings = bindings
                        .into_iter()
                        .map(|(_, value)| {
                            let aloc = cpsc411::Aloc::fresh();
                            let value = Box::new(value);
                            let value = uniquify_value(value);

                            (aloc, value)
                        })
                        .collect::<HashMap<_, _>>();
                    let tail = uniquify_tail(tail);
                    let tail = Box::new(tail);

                    target::Tail::r#let { bindings, tail }
                },
            }
        }

        fn uniquify_value(value: Box<self::Value>) -> target::Value {
            match *value {
                self::Value::triv { triv } => {
                    let triv = uniquify_triv(triv);
                    target::Value::triv { triv }
                },
                self::Value::binop_triv_triv { triv1, triv2 } => {
                    let triv1 = uniquify_triv(triv1);
                    let triv2 = uniquify_triv(triv2);
                    target::Value::binop_triv_triv { triv1, triv2 }
                },
                self::Value::r#let { bindings, value } => {
                    let bindings = bindings
                        .into_iter()
                        .map(|(_, value)| {
                            let aloc = cpsc411::Aloc::fresh();
                            let value = Box::new(value);
                            let value = uniquify_value(value);

                            (aloc, value)
                        })
                        .collect::<HashMap<_, _>>();
                    let value = uniquify_value(value);
                    let value = Box::new(value);

                    target::Value::r#let { bindings, value }
                },
            }
        }

        fn uniquify_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::name { .. } => target::Triv::aloc {
                    aloc: cpsc411::Aloc::fresh(),
                },
            }
        }

        let p = uniquify_p(p);
        target::ValuesUniqueLang { p }
    }
}
