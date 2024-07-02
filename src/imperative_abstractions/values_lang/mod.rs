pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use self::data::*;
use crate::imperative_abstractions::values_unique_lang as target;
use crate::utils;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ValuesLang(pub self::P);

impl ValuesLang {
    /// ### Purpose:
    /// ...
    pub fn check_values_lang(self) -> Result<Self, String> {
        Ok(self)
    }

    /// ### Purpose:
    /// Compiles Values-lang v3 to Values-unique-lang v3 by resolving all
    /// lexical identifiers to abstract locations.
    pub fn uniquify(self) -> target::ValuesUniqueLang {
        type ValueEnv = utils::LevelledEnv<utils::Name, utils::Aloc>;
        type LambdaEnv = HashMap<utils::Name, utils::Label>;
        let Self(p) = self;

        fn uniquify_p(p: self::P) -> target::P {
            match p {
                self::P::module { lambdas, tail } => {
                    let env = ValueEnv::default();
                    let lambda_env = collect_lambdas(&lambdas);
                    let lambdas = uniquify_lambdas(lambdas, &lambda_env);
                    let (tail, _) = uniquify_tail(tail, env, &lambda_env);
                    target::P::module { lambdas, tail }
                },
            }
        }

        fn collect_lambdas(lambdas: &Vec<self::Lambda>) -> LambdaEnv {
            let length = lambdas.len();
            lambdas.iter().fold(
                LambdaEnv::with_capacity(length),
                |mut lambda_env, Lambda { name, .. }| {
                    let label = utils::Label::new_with_name(*name);
                    lambda_env.insert(name.clone(), label);
                    lambda_env
                },
            )
        }

        fn uniquify_tail(
            tail: self::Tail,
            env: ValueEnv,
            lambda_env: &LambdaEnv,
        ) -> (target::Tail, ValueEnv) {
            match tail {
                self::Tail::value(value) => {
                    let (value, env) = uniquify_value(value, env);
                    let tail = target::Tail::value(value);
                    (tail, env)
                },
                self::Tail::r#let { bindings, tail } => {
                    let env = env.add_level();
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (tail, env) = uniquify_tail(*tail, env, lambda_env);
                    let tail = Box::new(tail);
                    let tail = target::Tail::r#let { bindings, tail };
                    let env = env.remove_level();
                    (tail, env)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let (pred, env) = uniquify_pred(pred, env);
                    let (tail1, env) = uniquify_tail(*tail1, env, lambda_env);
                    let (tail2, env) = uniquify_tail(*tail2, env, lambda_env);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    let tail = target::Tail::r#if { pred, tail1, tail2 };
                    (tail, env)
                },
                self::Tail::call { name, args } => {
                    let label = lambda_env.get(&name).unwrap().clone();
                    let triv = target::Triv::label(label);
                    let (opands, env) = uniquify_trivs(args, env);
                    let tail = target::Tail::call { triv, opands };
                    (tail, env)
                },
            }
        }

        fn uniquify_lambdas(
            lambdas: Vec<self::Lambda>,
            lambda_env: &LambdaEnv,
        ) -> Vec<target::Lambda> {
            lambdas
                .into_iter()
                .map(|lambda| uniquify_lambda(lambda, lambda_env))
                .collect()
        }

        fn uniquify_lambda(
            Lambda {
                name,
                tail,
                args: names,
            }: self::Lambda,
            lambda_env: &LambdaEnv,
        ) -> target::Lambda {
            let length = names.len();
            let (env, alocs) = names.into_iter().fold(
                (ValueEnv::with_capacity(length), vec![]),
                |(env, mut alocs), name| {
                    let aloc = utils::Aloc::fresh();
                    let env = env.insert(name, aloc.clone());
                    alocs.push(aloc);
                    (env, alocs)
                },
            );
            let label = lambda_env.get(&name).unwrap().clone();
            let (tail, _) = uniquify_tail(tail, env, lambda_env);
            target::Lambda {
                label,
                args: alocs,
                tail,
            }
        }

        fn uniquify_pred(
            pred: self::Pred,
            env: ValueEnv,
        ) -> (target::Pred, ValueEnv) {
            match pred {
                self::Pred::relop {
                    relop,
                    triv1,
                    triv2,
                } => {
                    let (opand1, env) = uniquify_triv(triv1, env);
                    let (opand2, env) = uniquify_triv(triv2, env);
                    let pred = target::Pred::relop {
                        relop,
                        opand1,
                        opand2,
                    };
                    (pred, env)
                },
                self::Pred::r#true => {
                    let pred = target::Pred::r#true;
                    (pred, env)
                },
                self::Pred::r#false => {
                    let pred = target::Pred::r#false;
                    (pred, env)
                },
                self::Pred::not(pred) => {
                    let (pred, env) = uniquify_pred(*pred, env);
                    let pred = Box::new(pred);
                    let pred = target::Pred::not(pred);
                    (pred, env)
                },
                self::Pred::r#let { bindings, pred } => {
                    let env = env.add_level();
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (pred, env) = uniquify_pred(*pred, env);
                    let pred = Box::new(pred);
                    let pred = target::Pred::r#let { bindings, pred };
                    let env = env.remove_level();
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
            env: ValueEnv,
        ) -> (target::Value, ValueEnv) {
            match value {
                self::Value::triv(triv) => {
                    let (opand, env) = uniquify_triv(triv, env);
                    let triv = target::Triv::opand(opand);
                    let value = target::Value::triv(triv);
                    (value, env)
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let (opand1, env) = uniquify_triv(triv1, env);
                    let (opand2, env) = uniquify_triv(triv2, env);
                    let value = target::Value::binop {
                        binop,
                        opand1,
                        opand2,
                    };
                    (value, env)
                },
                self::Value::r#let { bindings, value } => {
                    let env = env.add_level();
                    let (bindings, env) = uniquify_bindings(bindings, env);
                    let (value, env) = uniquify_value(*value, env);
                    let value = Box::new(value);
                    let value = target::Value::r#let { bindings, value };
                    let env = env.remove_level();
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

        fn uniquify_trivs(
            trivs: Vec<self::Triv>,
            env: ValueEnv,
        ) -> (Vec<target::Opand>, ValueEnv) {
            trivs
                .into_iter()
                .fold((vec![], env), |(mut opands, env), triv| {
                    let (opand, env) = uniquify_triv(triv, env);
                    opands.push(opand);
                    (opands, env)
                })
        }

        fn uniquify_triv(
            triv: self::Triv,
            env: ValueEnv,
        ) -> (target::Opand, ValueEnv) {
            match triv {
                self::Triv::int64(int64) => {
                    let opand = target::Opand::int64(int64);
                    (opand, env)
                },
                self::Triv::name(name) => {
                    let aloc = env.get(&name).map(utils::Aloc::clone).unwrap();
                    let opand = target::Opand::aloc(aloc);
                    (opand, env)
                },
            }
        }

        fn uniquify_bindings(
            bindings: self::Bindings,
            env: ValueEnv,
        ) -> (target::Bindings, ValueEnv) {
            let length = bindings.len();
            bindings.into_iter().fold(
                (HashMap::with_capacity(length), env),
                |(mut bindings, env), (name, value)| {
                    let (value, env) = uniquify_value(value, env);
                    let aloc = utils::Aloc::fresh();
                    bindings.insert(aloc.clone(), value);
                    let env = env.insert(name, aloc);
                    (bindings, env)
                },
            )
        }

        let p = uniquify_p(p);
        target::ValuesUniqueLang(p)
    }
}
