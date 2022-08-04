pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use self::data::*;
use crate::imperative_abstractions::values_unique_lang as target;
use crate::utils;

pub struct ValuesLang(pub self::P);

impl ValuesLang {
    /// ### Purpose:
    /// Compiles Values-lang v3 to Values-unique-lang v3 by resolving all
    /// lexical identifiers to abstract locations.
    pub fn uniquify(self) -> target::ValuesUniqueLang {
        type Env = HashMap<utils::Name, utils::Aloc>;
        let Self(p) = self;

        fn uniquify_p(p: self::P) -> target::P {
            let env = Env::new();

            match p {
                self::P::module(tail) => {
                    let (tail, _) = uniquify_tail(tail, env);
                    target::P::module(tail)
                },
            }
        }

        fn uniquify_tail(tail: self::Tail, env: Env) -> (target::Tail, Env) {
            match tail {
                self::Tail::value(value) => {
                    let (value, env) = uniquify_value(value, env);
                    (target::Tail::value(value), env)
                },
                self::Tail::r#let { bindings, tail } => {
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (tail, env) = uniquify_tail(*tail, env);
                    let tail = Box::new(tail);
                    (target::Tail::r#let { bindings, tail }, env)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let (pred, env) = uniquify_pred(pred, env);
                    let (tail1, env) = uniquify_tail(*tail1, env);
                    let (tail2, env) = uniquify_tail(*tail2, env);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    let tail = target::Tail::r#if { pred, tail1, tail2 };
                    (tail, env)
                },
            }
        }

        fn uniquify_pred(pred: self::Pred, env: Env) -> (target::Pred, Env) {
            match pred {
                self::Pred::relop {
                    relop,
                    triv1,
                    triv2,
                } => {
                    let (triv1, env) = uniquify_triv(triv1, env);
                    let (triv2, env) = uniquify_triv(triv2, env);
                    let pred = target::Pred::relop {
                        relop,
                        triv1,
                        triv2,
                    };
                    (pred, env)
                },
                self::Pred::r#true => (target::Pred::r#true, env),
                self::Pred::r#false => (target::Pred::r#false, env),
                self::Pred::not(pred) => {
                    let (pred, env) = uniquify_pred(*pred, env);
                    let pred = Box::new(pred);
                    let pred = target::Pred::not(pred);
                    (pred, env)
                },
                self::Pred::r#let { bindings, pred } => {
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (pred, env) = uniquify_pred(*pred, env);
                    let pred = Box::new(pred);
                    let pred = target::Pred::r#let { bindings, pred };
                    (pred, env)
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let (pred1, env) = uniquify_pred(*pred1, env);
                    let (pred2, env) = uniquify_pred(*pred2, env);
                    let (pred3, env) = uniquify_pred(*pred3, env);
                    let pred1 = Box::new(pred1);
                    let pred2 = Box::new(pred2);
                    let pred3 = Box::new(pred3);
                    let pred = target::Pred::r#if {
                        pred1,
                        pred2,
                        pred3,
                    };
                    (pred, env)
                },
            }
        }

        fn uniquify_value(
            value: self::Value,
            env: Env,
        ) -> (target::Value, Env) {
            match value {
                self::Value::triv(triv) => {
                    let (triv, env) = uniquify_triv(triv, env);
                    let value = target::Value::triv(triv);
                    (value, env)
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let (triv1, env) = uniquify_triv(triv1, env);
                    let (triv2, env) = uniquify_triv(triv2, env);
                    (
                        target::Value::binop_triv_triv {
                            binop,
                            triv1,
                            triv2,
                        },
                        env,
                    )
                },
                self::Value::r#let { bindings, value } => {
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (value, env) = uniquify_value(*value, env);
                    let value = Box::new(value);
                    let value = target::Value::r#let { bindings, value };
                    (value, env)
                },
                self::Value::r#if {
                    pred,
                    value1,
                    value2,
                } => {
                    let (pred, env) = uniquify_pred(pred, env);
                    let (value1, env) = uniquify_value(*value1, env);
                    let (value2, env) = uniquify_value(*value2, env);
                    let value1 = Box::new(value1);
                    let value2 = Box::new(value2);
                    let value = target::Value::r#if {
                        pred,
                        value1,
                        value2,
                    };
                    (value, env)
                },
            }
        }

        fn uniquify_triv(triv: self::Triv, env: Env) -> (target::Triv, Env) {
            match triv {
                self::Triv::int64(int64) => (target::Triv::int64(int64), env),
                self::Triv::name(name) => {
                    let aloc = env.get(&name).map(utils::Aloc::clone).unwrap();
                    let triv = target::Triv::aloc(aloc);
                    (triv, env)
                },
            }
        }

        fn uniquify_bindings(
            bindings: self::Bindings,
            env: Env,
        ) -> (target::Bindings, Env) {
            let length = bindings.len();

            let (mut env, bindings) = bindings.into_iter().fold(
                (env, HashMap::with_capacity(length)),
                |(env, mut bindings), (name, value)| {
                    let (value, env) = uniquify_value(value, env);
                    bindings.insert(name, value);
                    (env, bindings)
                },
            );

            let bindings = bindings
                .into_iter()
                .map(|(name, value)| {
                    let aloc = utils::Aloc::fresh();
                    env.insert(name, aloc.clone());
                    (aloc, value)
                })
                .collect();

            (bindings, env)
        }

        let p = uniquify_p(p);
        target::ValuesUniqueLang(p)
    }
}
