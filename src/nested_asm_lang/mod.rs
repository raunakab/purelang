pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::block_pred_lang as target;
use crate::cpsc411;

pub struct NestedAsmLang(pub self::P);

impl NestedAsmLang {
    /// OptimizePredicates: NestedAsmLang -> NestedAsmLang
    ///
    /// ### Purpose:
    /// Optimize Nested-asm-lang v4 programs by analyzing and simplifying
    /// predicates.
    pub fn optimize_predicates(self) -> Self {
        self
    }

    /// ExposeBasicBlocks: NestedAsmLang -> BlockPredLang
    ///
    /// ### Purpose:
    /// Compile the Nested-asm-lang v4 to Block-pred-lang v4, eliminating all
    /// nested expressions by generating fresh basic blocks and jumps.
    pub fn expose_basic_blocks(self) -> target::BlockPredLang {
        let Self(p) = self;

        fn expose_p(p: self::P) -> target::P {
            match p {
                self::P::module(tail) => {
                    let bs = vec![];

                    let (tail, mut bs) = expose_tail(tail, bs);

                    let label = cpsc411::Label::new_with_name("main");

                    let b = target::B::define { label, tail };

                    bs.push(b);

                    target::P::module(bs)
                },
            }
        }

        fn expose_tail(
            tail: self::Tail,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match tail {
                self::Tail::halt(triv) => (target::Tail::halt(triv), bs),

                self::Tail::begin { effects, tail } => {
                    let (tail, bs) = expose_tail(*tail, bs);

                    expose_effects(effects, tail, bs)
                },

                self::Tail::r#if { pred, tail1, tail2 } => {
                    let label_true = cpsc411::Label::new_with_name("true");

                    let label_false = cpsc411::Label::new_with_name("false");

                    let (tail1, bs) = expose_tail(*tail1, bs);

                    let (tail2, mut bs) = expose_tail(*tail2, bs);

                    let b1 = target::B::define {
                        label: label_true.clone(),
                        tail: tail1,
                    };

                    let b2 = target::B::define {
                        label: label_false.clone(),
                        tail: tail2,
                    };

                    bs.push(b1);

                    bs.push(b2);

                    expose_pred(pred, label_true, label_false, bs)
                },
            }
        }

        fn expose_effect(
            effect: self::Effect,
            tail: target::Tail,
            mut bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match effect {
                self::Effect::set { loc, triv } => {
                    let triv = target::Triv::opand(triv);

                    let effect = target::Effect::set { loc, triv };

                    let tail = make_begin(vec![effect], tail);

                    (tail, bs)
                },

                self::Effect::set_binop { loc, binop, triv } => {
                    let effect = target::Effect::set_binop {
                        loc,
                        binop,
                        opand: triv,
                    };

                    let tail = make_begin(vec![effect], tail);

                    (tail, bs)
                },

                self::Effect::begin(effects) => {
                    expose_effects(effects, tail, bs)
                },

                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let r#final = cpsc411::Label::new_with_name("final");

                    let b = target::B::define {
                        label: r#final.clone(),
                        tail,
                    };

                    bs.push(b);

                    let tail = target::Tail::jump(target::Trg::label(r#final));

                    let (tail1, bs) = expose_effect(*effect1, tail.clone(), bs);

                    let (tail2, mut bs) = expose_effect(*effect2, tail, bs);

                    let label_true = cpsc411::Label::new_with_name("true");

                    let label_false = cpsc411::Label::new_with_name("false");

                    let b1 = target::B::define {
                        label: label_true.clone(),
                        tail: tail1,
                    };

                    let b2 = target::B::define {
                        label: label_false.clone(),
                        tail: tail2,
                    };

                    bs.push(b1);

                    bs.push(b2);

                    expose_pred(pred, label_true, label_false, bs)
                },
            }
        }

        fn expose_effects(
            effects: Vec<self::Effect>,
            tail: target::Tail,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            effects.into_iter().fold((tail, bs), |(tail, bs), effect| {
                expose_effect(effect, tail, bs)
            })
        }

        fn expose_pred(
            pred: self::Pred,
            label_true: cpsc411::Label,
            label_false: cpsc411::Label,
            bs: Vec<target::B>,
        ) -> (target::Tail, Vec<target::B>) {
            match pred {
                self::Pred::r#true => {
                    (target::Tail::jump(target::Trg::label(label_true)), bs)
                },

                self::Pred::r#false => {
                    (target::Tail::jump(target::Trg::label(label_false)), bs)
                },

                self::Pred::not(pred) => {
                    expose_pred(*pred, label_false, label_true, bs)
                },

                self::Pred::relop { relop, loc, triv } => {
                    let pred = target::Pred::relop {
                        relop,
                        loc,
                        opand: triv,
                    };

                    let tail = target::Tail::r#if {
                        pred,
                        trg1: target::Trg::label(label_true),
                        trg2: target::Trg::label(label_false),
                    };

                    (tail, bs)
                },

                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let l_true = cpsc411::Label::new_with_name("true");

                    let l_false = cpsc411::Label::new_with_name("false");

                    let (tail, bs) = expose_pred(
                        *pred1,
                        l_true.clone(),
                        l_false.clone(),
                        bs,
                    );

                    let (tail2, bs) = expose_pred(
                        *pred2,
                        label_true.clone(),
                        label_false.clone(),
                        bs,
                    );

                    let (tail3, mut bs) =
                        expose_pred(*pred3, label_true, label_false, bs);

                    let b2 = target::B::define {
                        label: l_true,
                        tail: tail2,
                    };

                    let b3 = target::B::define {
                        label: l_false,
                        tail: tail3,
                    };

                    bs.push(b2);

                    bs.push(b3);

                    (tail, bs)
                },

                self::Pred::begin { effects, pred } => {
                    let label_true = cpsc411::Label::new_with_name("true");

                    let label_false = cpsc411::Label::new_with_name("false");

                    let (tail, bs) =
                        expose_pred(*pred, label_true, label_false, bs);

                    expose_effects(effects, tail, bs)
                },
            }
        }

        let p = expose_p(p);

        target::BlockPredLang(p)
    }

    // pub fn expose_basic_blocks(self) -> target::BlockPredLang {
    //     let Self(p) = self;
    //     fn expose_p(p: self::P) -> target::P {
    //         todo!()
    //     }
    //     fn expose_effects(
    //         effects: Vec<self::Effect>,
    //         tail: target::Tail,
    //         bs: Vec<target::B>,
    //     ) -> (target::Tail, Vec<target::B>) {
    //         fn generate_finish_label() -> cpsc411::Label {
    // cpsc411::Label::new_with_name("finish") }         // let tail =
    // effects         //     .into_iter()
    //         //     .fold_while::<(Option<u8>, cpsc411::Label,
    // Vec<target::B>), _>((None, generate_finish_label(), bs), |(tail,
    // finish_label, bs), effect| {         //         let x =
    // expose_effect(effect, finish_label, bs);         //         todo!()
    //         //     })
    //         //     .into_inner();
    //         let x = effects
    //             .into_iter()
    //             .fold((vec![], (generate_finish_label(),
    // generate_finish_label()), bs), |(mut effects, (prev_finish_label,
    // finish_label), bs), effect| {                 let (result, mut bs) =
    // expose_effect(effect, finish_label.clone(), bs);                 let
    // (effects, prev_finish_label, finish_label) = match result {
    //                     Either::Left(tail) => {
    //                         let length = effects.len();
    //                         let tail = match length {
    //                             0 => tail,
    //                             _ => target::Tail::begin { effects, tail:
    // Box::new(tail) },                         };
    //                         let b = target::B::define { label:
    // prev_finish_label, tail };                         bs.push(b);
    //                         (vec![], finish_label, generate_finish_label())
    //                     },
    //                     Either::Right(mapped_effects) => {
    //                         effects.extend(mapped_effects);
    //                         (effects, prev_finish_label, finish_label)
    //                     },
    //                 };
    //                 (effects, (prev_finish_label, finish_label), bs)
    //             });
    //         todo!()
    //     }
    //     fn expose_effect(
    //         effect: self::Effect,
    //         finish_label: cpsc411::Label,
    //         bs: Vec<target::B>,
    //     ) -> (Either<target::Tail, Vec<target::Effect>>, Vec<target::B>)
    //     {
    //         match effect {
    //             self::Effect::set { loc, triv } => {
    //                 let effect = target::Effect::set {
    //                     loc,
    //                     triv: target::Triv::opand(triv),
    //                 };
    //                 (Either::Right(vec![effect]), bs)
    //             },
    //             self::Effect::set_binop { loc, binop, triv } => {
    //                 let effect = target::Effect::set_binop {
    //                     loc,
    //                     binop,
    //                     opand: triv,
    //                 };
    //                 (Either::Right(vec![effect]), bs)
    //             },
    //             self::Effect::begin(..) => todo!(),
    //             self::Effect::r#if {
    //                 pred,
    //                 effect1,
    //                 effect2,
    //             } => {
    //                 let label1 = cpsc411::Label::new_with_name("label1");
    //                 let label2 = cpsc411::Label::new_with_name("label2");
    //                 fn apply(
    //                     effect: self::Effect,
    //                     label: cpsc411::Label,
    //                     finish_label: cpsc411::Label,
    //                     bs: Vec<target::B>,
    //                 ) -> Vec<target::B> {
    //                     let (result, mut bs) =
    //                         expose_effect(effect, finish_label.clone(), bs);
    //                     let tail = match result {
    //                         Either::Left(tail) => tail,
    //                         Either::Right(effects) => target::Tail::begin {
    //                             effects,
    //                             tail: Box::new(target::Tail::jump(
    //                                 target::Trg::label(finish_label),
    //                             )),
    //                         },
    //                     };
    //                     let b = target::B::define { label, tail };
    //                     bs.push(b);
    //                     bs
    //                 }
    //                 let bs = apply(
    //                     *effect1,
    //                     label1.clone(),
    //                     finish_label.clone(),
    //                     bs,
    //                 );
    //                 let bs = apply(*effect2, label2.clone(), finish_label,
    // bs);                 let trg1 = target::Trg::label(label1);
    //                 let trg2 = target::Trg::label(label2);
    //                 let (tail, bs) = expose_pred(pred, trg1, trg2, bs);
    //                 (Either::Left(tail), bs)
    //             },
    //         }
    //     }
    //     fn expose_pred(
    //         pred: self::Pred,
    //         trg1: target::Trg,
    //         trg2: target::Trg,
    //         mut bs: Vec<target::B>,
    //     ) -> (target::Tail, Vec<target::B>) {
    //         match pred {
    //             self::Pred::relop { relop, loc, triv } => {
    //                 let pred = target::Pred::relop {
    //                     relop,
    //                     loc,
    //                     opand: triv,
    //                 };
    //                 let tail = target::Tail::r#if { pred, trg1, trg2 };
    //                 (tail, bs)
    //             },
    //             self::Pred::r#true => {
    //                 let tail = target::Tail::jump(trg1);
    //                 (tail, bs)
    //             },
    //             self::Pred::r#false => {
    //                 let tail = target::Tail::jump(trg2);
    //                 (tail, bs)
    //             },
    //             self::Pred::not(pred) => expose_pred(*pred, trg2, trg1, bs),
    //             self::Pred::r#if {
    //                 pred1,
    //                 pred2,
    //                 pred3,
    //             } => {
    //                 let label1 = cpsc411::Label::new_with_name("trg1");
    //                 let label2 = cpsc411::Label::new_with_name("trg2");
    //                 let (tail1, bs) =
    //                     expose_pred(*pred2, trg1.clone(), trg2.clone(), bs);
    //                 let (tail2, mut bs) = expose_pred(*pred3, trg1, trg2,
    // bs);                 let b1 = target::B::define {
    //                     label: label1.clone(),
    //                     tail: tail1,
    //                 };
    //                 let b2 = target::B::define {
    //                     label: label2.clone(),
    //                     tail: tail2,
    //                 };
    //                 bs.push(b1);
    //                 bs.push(b2);
    //                 let new_trg1 = target::Trg::label(label1);
    //                 let new_trg2 = target::Trg::label(label2);
    //                 expose_pred(*pred1, new_trg1, new_trg2, bs)
    //             },
    //             self::Pred::begin { effects, pred } => {
    //                 let effects = vec![];
    //                 let (tail, bs) = expose_pred(*pred, trg1, trg2, bs);
    //                 let tail = target::Tail::begin {
    //                     effects,
    //                     tail: Box::new(tail),
    //                 };
    //                 (tail, bs)
    //             },
    //         }
    //     }
    //     let p = expose_p(p);
    //     target::BlockPredLang(p)
    // }

    // pub fn expose_basic_blocks(self) -> target::BlockPredLang {
    //     let Self(p) = self;
    //     fn expose_p(_: self::P) -> target::P { todo!() }
    //     fn expose_tail(
    //         tail: self::Tail,
    //         label: cpsc411::Label,
    //         bs: Vec<target::B>,
    //     ) -> (target::Tail, Vec<target::B>) {
    //         match tail {
    //             self::Tail::halt(triv) => {
    //                 let tail = target::Tail::halt(triv);
    //                 (tail, bs)
    //             },
    //             self::Tail::begin { effects, tail } => {
    //                 let (effects, bs) = expose_effects(effects,
    // label.clone(), bs);                 let (tail, bs) =
    // expose_tail(*tail, label, bs);                 let tail =
    // target::Tail::begin { effects, tail: Box::new(tail) };
    // (tail, bs)             },
    //             self::Tail::r#if { pred, tail1, tail2 } => {
    //                 let pred = target::Pred::r#true;
    //                 let label1 = cpsc411::Label::new_with_name("trg1");
    //                 let label2 = cpsc411::Label::new_with_name("trg2");
    //                 let (tail1, bs) = expose_tail(*tail1, label.clone(), bs);
    //                 let (tail2, mut bs) = expose_tail(*tail2, label, bs);
    //                 let b1 = target::B::define { label: label1.clone(), tail:
    // tail1 };                 let b2 = target::B::define { label:
    // label2.clone(), tail: tail2 };                 bs.push(b1);
    //                 bs.push(b2);
    //                 let trg1 = target::Trg::label(label1);
    //                 let trg2 = target::Trg::label(label2);
    //                 let tail = target::Tail::r#if { pred, trg1, trg2 };
    //                 (tail, bs)
    //             },
    //         }
    //     }
    //     fn expose_effects(
    //         effects: Vec<Effect>,
    //         label: cpsc411::Label,
    //         mut bs: Vec<target::B>,
    //     ) -> (Vec<target::Effect>, Vec<target::B>) {
    //         effects
    //             .into_iter()
    //             .fold((vec![], bs), |(mut effects, bs), effect| {
    //                 let (new_effects, bs) = expose_effect(effect,
    // label.clone(), bs);                 effects.extend(new_effects);
    //                 (effects, bs)
    //             })
    //     }
    //     fn expose_effect(
    //         effect: self::Effect,
    //         label: cpsc411::Label,
    //         mut bs: Vec<target::B>,
    //     ) -> (Vec<target::Effect>, Vec<target::B>) {
    //         match effect {
    //             self::Effect::set { loc, triv } => {
    //                 todo!()
    //             },
    //             self::Effect::set_binop { loc, binop, triv } => {
    //                 let effect = target::Effect::set_binop { loc, binop,
    // opand: triv };                 (vec![effect], bs)
    //             },
    //             self::Effect::begin(effects) => expose_effects(effects,
    // label, bs),             self::Effect::r#if { pred, effect1, effect2 }
    // => {                 todo!()
    //             },
    //         }
    //     }
    //     fn expose_pred(
    //         pred: self::Pred,
    //         label: cpsc411::Label,
    //         mut bs: Vec<target::B>,
    //     ) -> (Vec<target::Effect>, Vec<target::B>) {
    //         match pred {
    //             _ => todo!(),
    //         }
    //     }
    //     let p = expose_p(p);
    //     target::BlockPredLang(p)
    // }

    // /// FlattenBegins: NestedAsmLang -> ParaAsmLang
    // ///
    // /// ### Purpose:
    // /// Flatten all nested begin expressions.
    // pub fn flatten_begins(self) -> target::ParaAsmLang {
    //     let Self { p } = self;
    //     fn flatten_p(p: self::P) -> target::P {
    //         match p {
    //             self::P::tail { tail } => {
    //                 todo!()
    //                 // let tail = Box::new(tail);
    //                 // flatten_tail(tail)
    //             },
    //         }
    //     }
    //     fn flatten_tail(tail: Box<self::Tail>) -> target::P {
    //         match *tail {
    //             self::Tail::begin { effects, tail } => {
    //                 // let mut effects = flatten_effects(effects);
    //                 // let target::P::begin {
    //                 //     effects: tail_effects,
    //                 //     halt,
    //                 // } = flatten_tail(tail);
    //                 // effects.extend(tail_effects);
    //                 // target::P::begin { effects, halt }
    //                 todo!()
    //             },
    //             self::Tail::halt { triv } => {
    //                 // let triv = flatten_triv(triv);
    //                 // let halt = target::Halt { triv };
    //                 // target::P::begin {
    //                 //     effects: vec![],
    //                 //     halt,
    //                 // }
    //                 todo!()
    //             },
    //         }
    //     }
    //     fn flatten_effects(effects: Vec<self::Effect>) -> Vec<target::S> {
    //         effects
    //             .into_iter()
    //             .map(flatten_effect)
    //             .flatten()
    //             .collect::<Vec<_>>()
    //     }
    //     fn flatten_effect(effect: self::Effect) -> Vec<target::S> {
    //         match effect {
    //             self::Effect::set_loc_triv { loc, triv } => {
    //                 let loc = flatten_loc(loc);
    //                 let triv = flatten_triv(triv);
    //                 let effect = target::S::set_loc_triv { loc, triv };
    //                 vec![effect]
    //             },
    //             self::Effect::set_loc_binop_triv { loc, binop, triv } => {
    //                 // let loc = flatten_loc(loc);
    //                 // let triv = flatten_triv(triv);
    //                 // let effect =
    //                 //     target::S::set_loc_binop_opand { loc, binop, triv
    // };                 // vec![effect]
    //                 todo!()
    //             },
    //             self::Effect::begin { effects } => effects
    //                 .into_iter()
    //                 .map(flatten_effect)
    //                 .flatten()
    //                 .collect::<Vec<_>>(),
    //         }
    //     }
    //     fn flatten_triv(triv: self::Triv) -> target::Triv {
    //         match triv {
    //             self::Triv::loc { loc } => {
    //                 // let loc = flatten_loc(loc);
    //                 // target::Triv::loc { loc }
    //                 todo!()
    //             },
    //             self::Triv::int64 { int64 } => {
    //                 // target::Triv::int64 { int64 }
    //                 todo!()
    //             },
    //         }
    //     }
    //     fn flatten_loc(loc: self::Loc) -> target::Loc {
    //         match loc {
    //             self::Loc::fvar { fvar } => target::Loc::fvar { fvar },
    //             self::Loc::reg { reg } => target::Loc::reg { reg },
    //         }
    //     }
    //     let p = flatten_p(p);
    //     target::ParaAsmLang { p }
    // }
}

/// MakeBegin: Vec<Effect> Tail -> Tail
///
/// ### Purpose:
/// Tries to compress nested begins as much as possible.
fn make_begin(
    effects: Vec<target::Effect>,
    tail: target::Tail,
) -> target::Tail {
    let length = effects.len();

    match length {
        0 => tail,
        _ => match tail {
            target::Tail::begin {
                effects: mut tail_effects,
                tail,
            } => {
                tail_effects.extend(effects);

                target::Tail::begin {
                    effects: tail_effects,
                    tail,
                }
            },

            _ => target::Tail::begin {
                effects,
                tail: Box::new(tail),
            },
        },
    }
}
