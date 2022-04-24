pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use self::data::*;
use crate::cpsc411;
use crate::cpsc411::Compile;
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
        type Env = HashMap<self::Name, cpsc411::Aloc>;
        let mut env = Env::new();

        let Self { p } = self;

        fn uniquify_p(p: self::P, env: &mut Env) -> target::P {
            match p {
                self::P::module { tail } => {
                    let tail = Box::new(tail);
                    let tail = uniquify_tail(tail, env);

                    target::P::module { tail }
                },
            }
        }

        fn uniquify_tail(tail: Box<self::Tail>, env: &mut Env) -> target::Tail {
            match *tail {
                self::Tail::value { value } => {
                    let value = Box::new(value);
                    let value = uniquify_value(value, env);

                    target::Tail::value { value }
                },
                self::Tail::r#let { bindings, tail } => {
                    let bindings = uniquify_bindings(bindings, env);
                    let tail = uniquify_tail(tail, env);
                    let tail = Box::new(tail);

                    target::Tail::r#let { bindings, tail }
                },
                _ => todo!(),
            }
        }

        fn uniquify_value(
            value: Box<self::Value>,
            env: &mut Env,
        ) -> target::Value {
            match *value {
                self::Value::triv { triv } => {
                    let triv = uniquify_triv(triv, env);
                    target::Value::triv { triv }
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let triv1 = uniquify_triv(triv1, env);
                    let triv2 = uniquify_triv(triv2, env);

                    target::Value::binop_triv_triv {
                        binop,
                        triv1,
                        triv2,
                    }
                },
                self::Value::r#let { bindings, value } => {
                    let bindings = uniquify_bindings(bindings, env);
                    let value = uniquify_value(value, env);
                    let value = Box::new(value);

                    target::Value::r#let { bindings, value }
                },
                _ => todo!(),
            }
        }

        fn uniquify_triv(triv: self::Triv, env: &mut Env) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::name { name } => {
                    let aloc =
                        env.get(&name).map(cpsc411::Aloc::clone).unwrap();

                    target::Triv::aloc { aloc }
                },
            }
        }

        fn uniquify_bindings(
            bindings: HashMap<self::Name, self::Value>,
            env: &mut Env,
        ) -> HashMap<cpsc411::Aloc, target::Value> {
            let bindings = bindings
                .into_iter()
                .map(|(name, value)| {
                    let value = Box::new(value);
                    let value = uniquify_value(value, env);

                    (name, value)
                })
                .collect::<HashMap<_, _>>();

            bindings
                .into_iter()
                .map(|(name, value)| {
                    let aloc = cpsc411::Aloc::fresh();
                    env.insert(name, aloc.clone());

                    (aloc, value)
                })
                .collect::<HashMap<_, _>>()
        }

        let p = uniquify_p(p, &mut env);
        target::ValuesUniqueLang { p }
    }
}

impl Compile for ValuesLang {
    fn compile(
        self,
        opt_level: crate::OptLevels,
    ) -> crate::paren_x64::ParenX64 {
        self.uniquify()
            .sequentialize_let()
            .normalize_bind()
            .select_instructions()
            .compile(opt_level)
    }
}
