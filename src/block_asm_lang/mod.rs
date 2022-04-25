pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::para_asm_lang as target;

pub struct BlockAsmLang {
    pub p: self::P,
}

impl BlockAsmLang {
    /// FlattenProgram: BlockAsmLang -> ParaAsmLang
    ///
    /// ### Purpose:
    /// Compile Block-asm-lang v4 to Para-asm-lang v4 by flattening basic blocks
    /// into labeled instructions.
    pub fn flatten_program(self) -> target::ParaAsmLang {
        let Self { p } = self;

        fn flatten_p(p: self::P) -> target::P {
            match p {
                self::P::module { bs } => {
                    let ss = flatten_bs(bs);
                    target::P::begin { ss }
                },
            }
        }

        fn flatten_bs(bs: Vec<self::B>) -> Vec<target::S> {
            bs.into_iter().map(flatten_b).flatten().collect()
        }

        fn flatten_b(b: self::B) -> Vec<target::S> {
            match b {
                self::B::define_label_tail { label, tail } => {
                    flatten_tail(tail)
                        .into_iter()
                        .enumerate()
                        .map(|(index, s)| match index {
                            0usize => target::S::with_label {
                                label: label.clone(),
                                s: Box::new(s),
                            },
                            _ => s,
                        })
                        .collect()
                },
            }
        }

        fn flatten_tail(tail: self::Tail) -> Vec<target::S> {
            match tail {
                self::Tail::halt { opand } => vec![target::S::halt { opand }],
                self::Tail::jump { trg } => vec![target::S::jump { trg }],
                self::Tail::begin { effects, tail } => {
                    let mut ss = flatten_effects(effects);
                    let ss_tail = flatten_tail(*tail);

                    ss.extend(ss_tail);
                    ss
                },
                self::Tail::r#if {
                    relop,
                    loc,
                    opand,
                    trg1,
                    trg2,
                } => {
                    let instr1 = target::S::compare_jump {
                        loc,
                        opand,
                        relop,
                        trg: trg1,
                    };
                    let instr2 = target::S::jump { trg: trg2 };
                    vec![instr1, instr2]
                },
            }
        }

        fn flatten_effects(effects: Vec<self::Effect>) -> Vec<target::S> {
            effects.into_iter().map(flatten_effect).collect()
        }

        fn flatten_effect(effect: self::Effect) -> target::S {
            match effect {
                self::Effect::set { loc, triv } => {
                    target::S::set_loc_triv { loc, triv }
                },
                self::Effect::set_binop { loc, binop, opand } => {
                    target::S::set_loc_binop_opand { loc, binop, opand }
                },
            }
        }

        let p = flatten_p(p);
        target::ParaAsmLang { p }
    }
}
