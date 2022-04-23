pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::asm_lang as target;
use crate::cpsc411;

#[derive(Debug, PartialEq, Eq)]
pub struct ImpCmfLang {
    pub p: self::P,
}

impl ImpCmfLang {
    /// SelectInstructions: ImpCmfLang -> AsmLang
    ///
    /// ### Purpose:
    /// Compiles Imp-cmf-lang v3 to Asm-lang v2, selecting appropriate sequences
    /// of abstract assembly instructions to implement the operations of the
    /// source language.
    pub fn select_instructions(self) -> target::AsmLang {
        let Self { p } = self;

        fn select_p(p: self::P) -> target::P {
            match p {
                self::P::module { tail } => {
                    let tail = Box::new(tail);
                    let tail = select_tail(tail);

                    target::P::module {
                        info: cpsc411::Info::default(),
                        tail,
                    }
                },
            }
        }

        fn select_tail(tail: Box<self::Tail>) -> target::Tail {
            match *tail {
                self::Tail::value { value } => select_value_from_tail(value),
                self::Tail::begin { effects, tail } => {
                    let mut effects = select_effects(effects);
                    let tail = select_tail(tail);

                    match tail {
                        target::Tail::halt { triv } => {
                            let tail = target::Tail::halt { triv };
                            let tail = Box::new(tail);

                            target::Tail::begin { effects, tail }
                        },
                        target::Tail::begin {
                            effects: tail_effects,
                            tail,
                        } => {
                            effects.extend(tail_effects);

                            target::Tail::begin { effects, tail }
                        },
                    }
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
                self::Effect::set_aloc_value { aloc, value } => {
                    let effect = select_value(value, aloc);
                    match effect {
                        target::Effect::set_aloc_triv { .. } => vec![effect],
                        target::Effect::set_aloc_binop_aloc_triv { .. } => {
                            vec![effect]
                        },
                        target::Effect::begin { effects } => effects,
                    }
                    // vec![effect]
                },
                self::Effect::begin { effects } => select_effects(effects),
            }
        }

        fn select_value(
            value: self::Value,
            aloc: cpsc411::Aloc,
        ) -> target::Effect {
            match value {
                self::Value::triv { triv } => {
                    let triv = select_triv(triv);
                    target::Effect::set_aloc_triv { aloc, triv }
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let triv1 = select_triv(triv1);
                    let triv2 = select_triv(triv2);

                    let instr1 = target::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: triv1,
                    };
                    let instr2 = target::Effect::set_aloc_binop_aloc_triv {
                        aloc,
                        binop,
                        triv: triv2,
                    };

                    let effects = vec![instr1, instr2];

                    target::Effect::begin { effects }
                },
            }
        }

        fn select_value_from_tail(value: self::Value) -> target::Tail {
            match value {
                self::Value::triv { triv } => {
                    let triv = select_triv(triv);
                    target::Tail::halt { triv }
                },
                self::Value::binop_triv_triv {
                    binop,
                    triv1,
                    triv2,
                } => {
                    let aloc = cpsc411::Aloc::fresh();

                    let triv1 = select_triv(triv1);
                    let triv2 = select_triv(triv2);

                    let instr1 = target::Effect::set_aloc_triv {
                        aloc: aloc.clone(),
                        triv: triv1,
                    };
                    let instr2 = target::Effect::set_aloc_binop_aloc_triv {
                        aloc: aloc.clone(),
                        binop,
                        triv: triv2,
                    };

                    let effects = vec![instr1, instr2];

                    let triv = target::Triv::aloc { aloc };
                    let tail = target::Tail::halt { triv };
                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
            }
        }

        fn select_triv(triv: self::Triv) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::aloc { aloc } => target::Triv::aloc { aloc },
            }
        }

        let p = select_p(p);
        target::AsmLang { p }
    }
}
