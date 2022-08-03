pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

pub use self::data::*;
use crate::utils;
use crate::structured_control_flow::nested_asm_lang as target;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct AsmLang(pub self::P);

impl AsmLang {
    /// ### Purpose:
    /// Compiles Asm-lang v2 to Asm-lang v2/locals, analysing which abstract
    /// locations are used in the program and decorating the program with the
    /// set of variables in an info field.
    fn uncover_locals(self) -> Self {
        let Self(p) = self;

        fn uncover_p(p: self::P) -> self::P {
            match p {
                self::P::module { tail, .. } => {
                    let locals = utils::AlocSet::default();
                    let locals = uncover_tail(&tail, locals);
                    let locals = Some(locals);
                    let info = utils::Info {
                        locals,
                        ..Default::default()
                    };
                    self::P::module { info, tail }
                },
            }
        }

        fn uncover_tail(
            tail: &self::Tail,
            locals: utils::AlocSet,
        ) -> utils::AlocSet {
            match tail {
                self::Tail::halt(triv) => uncover_triv(triv, locals),
                self::Tail::begin { effects, tail } => {
                    let locals = uncover_effects(effects, locals);
                    uncover_tail(&tail, locals)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let locals = uncover_pred(pred, locals);
                    let locals = uncover_tail(&tail1, locals);
                    uncover_tail(&tail2, locals)
                },
            }
        }

        fn uncover_pred(
            pred: &self::Pred,
            locals: utils::AlocSet,
        ) -> utils::AlocSet {
            match pred {
                self::Pred::relop { .. }
                | self::Pred::r#true
                | self::Pred::r#false => locals,
                self::Pred::begin { effects, pred } => {
                    let locals = uncover_effects(effects, locals);
                    uncover_pred(&pred, locals)
                },
                self::Pred::not(pred) => uncover_pred(&pred, locals),
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let locals = uncover_pred(&pred1, locals);
                    let locals = uncover_pred(&pred2, locals);
                    uncover_pred(&pred3, locals)
                },
            }
        }

        fn uncover_effects(
            effects: &Vec<self::Effect>,
            locals: utils::AlocSet,
        ) -> utils::AlocSet {
            effects
                .iter()
                .fold(locals, |locals, effect| uncover_effect(effect, locals))
        }

        fn uncover_effect(
            effect: &self::Effect,
            mut locals: utils::AlocSet,
        ) -> utils::AlocSet {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv }
                | self::Effect::set_aloc_binop_aloc_triv {
                    aloc, triv, ..
                } => {
                    locals.insert(aloc.clone());
                    uncover_triv(triv, locals)
                },
                self::Effect::begin(effects) => {
                    uncover_effects(effects, locals)
                },
                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let locals = uncover_pred(pred, locals);
                    let locals = uncover_effect(&effect1, locals);
                    uncover_effect(&effect2, locals)
                },
            }
        }

        fn uncover_triv(
            triv: &self::Triv,
            mut locals: utils::AlocSet,
        ) -> utils::AlocSet {
            match triv {
                self::Triv::int64(..) => HashSet::new(),
                self::Triv::aloc(aloc) => {
                    locals.insert(aloc.clone());
                    locals
                },
            }
        }

        let p = uncover_p(p);
        Self(p)
    }

    /// ### Purpose:
    /// Compiles Asm-lang v2/locals to Asm-lang v2/assignments, by assigning
    /// each abstract location from the locals info field to a fresh frame
    /// variable.
    fn assign_fvars(self) -> Self {
        let Self(p) = self;

        fn assign_p(p: self::P) -> self::P {
            match p {
                self::P::module { info, tail } => {
                    let utils::Info { locals, .. } = info;
                    let locals = locals.unwrap();
                    let mut locals_as_vec = locals.iter().collect::<Vec<_>>();
                    locals_as_vec.sort();
                    let assignment = locals_as_vec
                        .into_iter()
                        .map(|aloc| {
                            let aloc = aloc.clone();
                            let fvar = utils::Fvar::fresh();
                            let loc = target::Loc::fvar(fvar);
                            (aloc, loc)
                        })
                        .collect();
                    let locals = Some(locals);
                    let assignment = Some(assignment);
                    let info = utils::Info {
                        locals,
                        assignment,
                        ..info /* undead_out,
                                * conflicts, */
                    };
                    self::P::module { info, tail }
                },
            }
        }

        let p = assign_p(p);
        Self(p)
    }

    /// ### Purpose:
    /// Performs undeadness analysis, decorating the program with undead-set
    /// tree. Only the info field of the program is modified.
    fn undead_analysis(self) -> Self {
        let Self(p) = self;

        fn undead_p(p: self::P) -> self::P {
            let ust = utils::Tree::new();
            match p {
                self::P::module { info, tail } => {
                    let last = utils::AlocSet::default();
                    let (utils::Tree { nodes }, _) =
                        undead_tail(&tail, ust, last);
                    let undead_out = nodes.get(0).unwrap().clone();
                    let undead_out = Some(undead_out);
                    let info = utils::Info { undead_out, ..info };
                    self::P::module { info, tail }
                },
            }
        }

        fn undead_tail(
            tail: &self::Tail,
            mut ust: utils::Tree,
            last: utils::AlocSet,
        ) -> (utils::Tree, utils::AlocSet) {
            match tail {
                self::Tail::halt(triv) => {
                    let node = utils::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);
                    let last = undead_triv(triv, last);
                    (ust, last)
                },
                self::Tail::begin { effects, tail } => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) = undead_tail(&tail, sub_ust, last);
                    let (sub_ust, last) =
                        undead_effects(effects, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) = undead_pred(pred, sub_ust, last);
                    let (sub_ust, last) = undead_tail(&tail1, sub_ust, last);
                    let (sub_ust, last) = undead_tail(&tail2, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },
            }
        }

        fn undead_pred(
            pred: &self::Pred,
            mut ust: utils::Tree,
            mut last: utils::AlocSet,
        ) -> (utils::Tree, utils::AlocSet) {
            match pred {
                self::Pred::begin { effects, pred } => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) =
                        undead_effects(effects, sub_ust, last);
                    let (sub_ust, last) = undead_pred(&pred, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },
                self::Pred::r#true | self::Pred::r#false => (ust, last),
                self::Pred::not(pred) => undead_pred(&pred, ust, last),
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) = undead_pred(&pred1, sub_ust, last);
                    let (sub_ust, last) = undead_pred(&pred2, sub_ust, last);
                    let (sub_ust, last) = undead_pred(&pred3, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },

                self::Pred::relop { aloc, triv, .. } => {
                    let node = utils::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);
                    last.remove(aloc);
                    let last = undead_triv(triv, last);
                    (ust, last)
                },
            }
        }

        fn undead_effects(
            effects: &Vec<self::Effect>,
            ust: utils::Tree,
            last: utils::AlocSet,
        ) -> (utils::Tree, utils::AlocSet) {
            effects.into_iter().rev().fold(
                (ust, last),
                |(curr_ust, curr_last), effect| {
                    undead_effect(effect, curr_ust, curr_last)
                },
            )
        }

        fn undead_effect(
            effect: &self::Effect,
            mut ust: utils::Tree,
            mut last: utils::AlocSet,
        ) -> (utils::Tree, utils::AlocSet) {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let node = utils::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);
                    last.remove(aloc);
                    let last = undead_triv(triv, last);
                    (ust, last)
                },
                self::Effect::set_aloc_binop_aloc_triv {
                    aloc, triv, ..
                } => {
                    let node = utils::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);
                    last.remove(aloc);
                    let last = undead_triv(triv, last);
                    (ust, last)
                },
                self::Effect::begin(effects) => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) =
                        undead_effects(effects, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },

                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let sub_ust = utils::Tree::new();
                    let (sub_ust, last) = undead_pred(pred, sub_ust, last);
                    let (sub_ust, last) =
                        undead_effect(&effect1, sub_ust, last);
                    let (sub_ust, last) =
                        undead_effect(&effect2, sub_ust, last);
                    let node = utils::Node::tree { tree: sub_ust };
                    ust.push_on(node);
                    (ust, last)
                },
            }
        }

        fn undead_triv(
            triv: &self::Triv,
            mut last: utils::AlocSet,
        ) -> utils::AlocSet {
            match triv {
                self::Triv::int64(..) => last,
                self::Triv::aloc(aloc) => {
                    let aloc = aloc.clone();
                    last.insert(aloc);
                    last
                },
            }
        }

        let p = undead_p(p);
        Self(p)
    }

    /// ### Purpose:
    /// Decorates a program with its conflict graph.
    fn conflict_analysis(self) -> Self {
        let Self(p) = self;

        fn conf_p(p: self::P) -> self::P {
            match p {
                self::P::module {
                    info:
                        utils::Info {
                            undead_out,
                            locals,
                            assignment,
                            ..
                        },
                    tail,
                } => {
                    let locals = locals.unwrap();
                    let undead_out = undead_out.unwrap();
                    let conflicts = utils::Graph::new(&locals);
                    let conflicts = conf_tail(&tail, &undead_out, conflicts);
                    let locals = Some(locals);
                    let undead_out = Some(undead_out);
                    let conflicts = Some(conflicts);
                    let info = utils::Info {
                        undead_out,
                        locals,
                        conflicts,
                        assignment,
                    };

                    self::P::module { info, tail }
                },
            }
        }

        fn conf_tail(
            tail: &self::Tail,
            ust: &utils::Node,
            conflicts: utils::Graph,
        ) -> utils::Graph {
            match tail {
                self::Tail::halt { .. } => conflicts,
                self::Tail::begin { effects, tail } => {
                    let conflicts = conf_tail(&tail, ust, conflicts);
                    conf_effects(effects, ust, conflicts)
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let conflicts = conf_pred(pred, ust, conflicts);
                    let conflicts = conf_tail(tail1, ust, conflicts);
                    conf_tail(tail2, ust, conflicts)
                },
            }
        }

        fn conf_pred(
            pred: &self::Pred,
            ust: &utils::Node,
            mut conflicts: utils::Graph,
        ) -> utils::Graph {
            match pred {
                self::Pred::begin { effects, pred } => {
                    let conflicts = conf_effects(effects, ust, conflicts);
                    conf_pred(pred, ust, conflicts)
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let conflicts = conf_pred(&pred1, ust, conflicts);
                    let conflicts = conf_pred(&pred2, ust, conflicts);
                    conf_pred(&pred3, ust, conflicts)
                },
                self::Pred::r#true | self::Pred::r#false => conflicts,
                self::Pred::not(pred) => conf_pred(&pred, ust, conflicts),
                self::Pred::relop { aloc, triv, .. } => {
                    let mut alocs = ust.to_alocs_panic().clone();
                    alocs.remove(aloc);
                    let alocs = remove_triv_from_alocs(triv, alocs);
                    conflicts.insert_alocs(aloc.clone(), alocs);
                    conflicts
                },
            }
        }

        fn conf_effects(
            effects: &Vec<self::Effect>,
            ust: &utils::Node,
            conflicts: utils::Graph,
        ) -> utils::Graph {
            let nodes = ust.to_tree_panic();
            effects.into_iter().zip(nodes).fold(
                conflicts,
                |curr_conflicts, (effect, node)| {
                    conf_effect(effect, node, curr_conflicts)
                },
            )
        }

        fn conf_effect(
            effect: &self::Effect,
            ust: &utils::Node,
            mut conflicts: utils::Graph,
        ) -> utils::Graph {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let mut alocs = ust.to_alocs_panic().clone();
                    alocs.remove(aloc);
                    let alocs = remove_triv_from_alocs(triv, alocs);
                    conflicts.insert_alocs(aloc.clone(), alocs);
                    conflicts
                },
                self::Effect::set_aloc_binop_aloc_triv {
                    aloc, triv, ..
                } => {
                    let mut alocs = ust.to_alocs_panic().clone();
                    alocs.remove(aloc);
                    let alocs = remove_triv_from_alocs(triv, alocs);
                    conflicts.insert_alocs(aloc.clone(), alocs);
                    conflicts
                },
                self::Effect::begin(effects) => {
                    conf_effects(effects, ust, conflicts)
                },
                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let conflicts = conf_pred(pred, ust, conflicts);
                    let conflicts = conf_effect(effect1, ust, conflicts);
                    conf_effect(effect2, ust, conflicts)
                },
            }
        }

        fn remove_triv_from_alocs(
            triv: &self::Triv,
            mut alocs: utils::AlocSet,
        ) -> utils::AlocSet {
            match triv {
                self::Triv::int64(..) => alocs,
                self::Triv::aloc(aloc) => {
                    alocs.remove(aloc);
                    alocs
                },
            }
        }

        let p = conf_p(p);
        Self(p)
    }

    /// ### Purpose:
    /// Performs graph-colouring register allocation. The pass attempts to fit
    /// each of the abstract location declared in the locals set into a
    /// register, and if one cannot be found, assigns it a frame variable
    /// instead.
    fn assign_registers(self) -> Self {
        let Self(p) = self;

        fn get_assignable_registers_from_assignments(
            assignments: &utils::Assignments<target::Loc>,
            current_assignable_registers: &HashSet<utils::Reg>,
            _: usize,
        ) -> Vec<utils::Reg> {
            let locs = assignments.values().collect::<HashSet<&target::Loc>>();
            let registers = current_assignable_registers
                .into_iter()
                .filter_map(|reg| {
                    let reg = *reg;
                    let loc = target::Loc::reg(reg);

                    let is_already_assigned = locs.contains(&loc);
                    match is_already_assigned {
                        true => None,
                        false => Some(reg),
                    }
                })
                .collect::<Vec<_>>();
            registers
        }

        fn lowest_order_aloc(
            utils::Graph { graph }: &utils::Graph,
        ) -> utils::Aloc {
            let mut conflicts = graph.into_iter().collect::<Vec<_>>();
            conflicts.sort_by(|(_, alocs1), (_, alocs2)| {
                let length1 = alocs1.len();
                let length2 = alocs2.len();
                length1.cmp(&length2)
            });
            let (aloc, _) = conflicts.first().unwrap();
            (*aloc).clone()
        }

        fn assign_p(p: self::P) -> self::P {
            let (current_assignable_registers, k) = {
                let registers = utils::Reg::current_assignable_registers();
                let length = registers.len();
                (registers, length)
            };

            match p {
                self::P::module { info, tail } => {
                    let utils::Info {
                        locals, conflicts, ..
                    } = info;
                    let locals = locals.unwrap();
                    let conflicts = conflicts.unwrap();
                    let assignment =
                        HashMap::<utils::Aloc, target::Loc>::default();
                    let assignment = recursive_assign(
                        locals,
                        conflicts,
                        assignment,
                        &current_assignable_registers,
                        k,
                    );
                    let assignment = Some(assignment);
                    let info = utils::Info {
                        locals: None,
                        conflicts: None,
                        assignment,
                        ..info
                    };
                    self::P::module { info, tail }
                },
            }
        }

        fn recursive_assign(
            mut locals: utils::AlocSet,
            mut conflicts: utils::Graph,
            assignments: utils::Assignments<target::Loc>,
            current_assignable_registers: &HashSet<utils::Reg>,
            k: usize,
        ) -> utils::Assignments<target::Loc> {
            let empty_locals = locals.is_empty();
            match empty_locals {
                true => assignments,
                false => {
                    let aloc = lowest_order_aloc(&conflicts);
                    locals.remove(&aloc);
                    conflicts.remove_node(&aloc);
                    let mut assignments = recursive_assign(
                        locals,
                        conflicts,
                        assignments,
                        current_assignable_registers,
                        k,
                    );
                    let loc = get_assignable_registers_from_assignments(
                        &assignments,
                        &current_assignable_registers,
                        k,
                    )
                    .get(0)
                    .map(|reg| target::Loc::reg(*reg))
                    .unwrap_or(target::Loc::fvar(utils::Fvar::fresh()));
                    assignments.insert(aloc, loc);
                    assignments
                },
            }
        }

        let p = assign_p(p);
        Self(p)
    }

    /// ### Purpose:
    /// Compiles Asm-lang v2/assignments to Nested-asm-lang v2, replaced each
    /// abstract location with its assigned physical location from the
    /// assignment info field.
    fn replace_locations(self) -> target::NestedAsmLang {
        let Self(p) = self;

        fn replace_p(p: self::P) -> target::P {
            match p {
                self::P::module {
                    info: utils::Info { assignment, .. },
                    tail,
                } => {
                    let assignment = assignment.unwrap();
                    let tail = Box::new(tail);
                    let tail = replace_tail(*tail, &assignment);
                    target::P::module(tail)
                },
            }
        }

        fn replace_tail(
            tail: self::Tail,
            assignment: &utils::Assignments<target::Loc>,
        ) -> target::Tail {
            match tail {
                self::Tail::halt(triv) => {
                    let triv = replace_triv(triv, assignment);
                    target::Tail::halt(triv)
                },
                self::Tail::begin { effects, tail } => {
                    let effects = replace_effects(effects, assignment);
                    let tail = replace_tail(*tail, assignment);
                    let tail = Box::new(tail);
                    target::Tail::begin { effects, tail }
                },
                self::Tail::r#if { pred, tail1, tail2 } => {
                    let pred = replace_pred(pred, assignment);
                    let tail1 = replace_tail(*tail1, assignment);
                    let tail2 = replace_tail(*tail2, assignment);
                    let tail1 = Box::new(tail1);
                    let tail2 = Box::new(tail2);
                    target::Tail::r#if { pred, tail1, tail2 }
                },
            }
        }

        fn replace_pred(
            pred: self::Pred,
            assignment: &utils::Assignments<target::Loc>,
        ) -> target::Pred {
            match pred {
                self::Pred::relop { relop, aloc, triv } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    let triv = replace_triv(triv, assignment);
                    target::Pred::relop { relop, loc, triv }
                },
                self::Pred::r#if {
                    pred1,
                    pred2,
                    pred3,
                } => {
                    let pred1 = replace_pred(*pred1, assignment);
                    let pred2 = replace_pred(*pred2, assignment);
                    let pred3 = replace_pred(*pred3, assignment);
                    let pred1 = Box::new(pred1);
                    let pred2 = Box::new(pred2);
                    let pred3 = Box::new(pred3);
                    target::Pred::r#if {
                        pred1,
                        pred2,
                        pred3,
                    }
                },
                self::Pred::not(pred) => replace_pred(*pred, assignment),
                self::Pred::r#true => target::Pred::r#true,
                self::Pred::r#false => target::Pred::r#false,
                self::Pred::begin { effects, pred } => {
                    let effects = replace_effects(effects, assignment);
                    let pred = replace_pred(*pred, assignment);
                    let pred = Box::new(pred);
                    target::Pred::begin { effects, pred }
                },
            }
        }

        fn replace_effects(
            effects: Vec<self::Effect>,
            assignment: &utils::Assignments<target::Loc>,
        ) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(|effect| replace_effect(effect, assignment))
                .collect::<Vec<_>>()
        }

        fn replace_effect(
            effect: self::Effect,
            assignment: &utils::Assignments<target::Loc>,
        ) -> target::Effect {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    let triv = replace_triv(triv, assignment);
                    target::Effect::set { loc, triv }
                },
                self::Effect::set_aloc_binop_aloc_triv {
                    aloc,
                    binop,
                    triv,
                } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    let triv = replace_triv(triv, assignment);
                    target::Effect::set_binop { loc, binop, triv }
                },
                self::Effect::begin(effects) => {
                    let effects = replace_effects(effects, assignment);
                    target::Effect::begin(effects)
                },
                self::Effect::r#if {
                    pred,
                    effect1,
                    effect2,
                } => {
                    let pred = replace_pred(pred, assignment);
                    let effect1 = replace_effect(*effect1, assignment);
                    let effect2 = replace_effect(*effect2, assignment);
                    let effect1 = Box::new(effect1);
                    let effect2 = Box::new(effect2);
                    target::Effect::r#if {
                        pred,
                        effect1,
                        effect2,
                    }
                },
            }
        }

        fn replace_triv(
            triv: self::Triv,
            assignment: &utils::Assignments<target::Loc>,
        ) -> target::Triv {
            match triv {
                self::Triv::int64(int64) => target::Triv::int64(int64),
                self::Triv::aloc(aloc) => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    target::Triv::loc(loc)
                },
            }
        }

        let p = replace_p(p);
        target::NestedAsmLang(p)
    }

    /// ### Purpose:
    /// Compiles Asm-lang v2 to Nested-asm-lang v2, replacing each abstract
    /// location with a physical location.
    pub fn assign_homes(self) -> target::NestedAsmLang {
        self.uncover_locals().assign_fvars().replace_locations()
    }

    /// ### Purpose:
    /// Compiles Asm-lang v2 to Nested-asm-lang v2, replacing each abstract
    /// location with a physical location. This version performs graph-colouring
    /// register allocation.
    pub fn assign_homes_opt(self) -> target::NestedAsmLang {
        self.uncover_locals()
            .undead_analysis()
            .conflict_analysis()
            .assign_registers()
            .replace_locations()
    }
}
