pub mod data;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;

pub use self::data::*;
use crate::cpsc411;
use crate::cpsc411::Compile;
use crate::nested_asm_lang as target;

#[derive(Debug, PartialEq, Eq)]
pub struct AsmLang {
    pub p: self::P,
}

impl AsmLang {
    /// UncoverLocals: Self -> Self/Locals
    ///
    /// ### Purpose:
    /// Compiles Asm-lang v2 to Asm-lang v2/locals, analysing which abstract
    /// locations are used in the program and decorating the program with the
    /// set of variables in an info field.
    fn uncover_locals(self) -> Self {
        let Self { p } = self;

        fn uncover_p(p: self::P) -> self::P {
            match p {
                self::P::module { tail, .. } => {
                    let locals = uncover_tail(&tail);
                    let locals = Some(locals);

                    let info = cpsc411::Info {
                        locals,
                        ..Default::default()
                    };

                    self::P::module { info, tail }
                },
            }
        }

        fn uncover_tail(tail: &self::Tail) -> cpsc411::AlocSet {
            match tail {
                self::Tail::halt { triv } => uncover_triv(triv),
                self::Tail::begin { effects, tail } => {
                    let mut locals_1 = uncover_effects(effects);
                    let locals_2 = uncover_tail(&tail);

                    locals_1.extend(locals_2);

                    locals_1
                },
            }
        }

        fn uncover_effects(effects: &Vec<self::Effect>) -> cpsc411::AlocSet {
            effects
                .iter()
                .map(uncover_effect)
                .flatten()
                .collect::<HashSet<_>>()
        }

        fn uncover_effect(effect: &self::Effect) -> cpsc411::AlocSet {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let mut locals_1 =
                        vec![aloc.clone()].into_iter().collect::<HashSet<_>>();
                    let locals_2 = uncover_triv(triv);

                    locals_1.extend(locals_2);

                    locals_1
                },

                self::Effect::set_aloc_binop_aloc_triv {
                    aloc, triv, ..
                } => {
                    let mut locals_1 =
                        vec![aloc.clone()].into_iter().collect::<HashSet<_>>();
                    let locals_2 = uncover_triv(triv);

                    locals_1.extend(locals_2);

                    locals_1
                },

                self::Effect::begin { effects } => uncover_effects(effects),
            }
        }

        fn uncover_triv(triv: &self::Triv) -> cpsc411::AlocSet {
            match triv {
                self::Triv::int64 { .. } => HashSet::new(),
                self::Triv::aloc { aloc } => {
                    vec![aloc.clone()].into_iter().collect::<HashSet<_>>()
                },
            }
        }

        let p = uncover_p(p);
        Self { p }
    }

    /// AssignFvars: Self/Locals -> Self/Assignments
    ///
    /// ### Purpose:
    /// Compiles Asm-lang v2/locals to Asm-lang v2/assignments, by assigning
    /// each abstract location from the locals info field to a fresh frame
    /// variable.
    fn assign_fvars(self) -> Self {
        let Self { p } = self;

        fn assign_p(p: self::P) -> self::P {
            match p {
                self::P::module {
                    info, /* cpsc411::Info {
                           *     locals,
                           *     undead_out,
                           *     conflicts,
                           *     ..
                           * } */
                    tail,
                } => {
                    let cpsc411::Info { locals, .. } = info;

                    let locals = locals.unwrap();

                    let mut locals_as_vec = locals.iter().collect::<Vec<_>>();
                    locals_as_vec.sort();

                    let assignment = locals_as_vec
                        .into_iter()
                        .map(|aloc| {
                            let aloc = aloc.clone();
                            let fvar = cpsc411::Fvar::fresh();
                            let loc = target::Loc::fvar { fvar };
                            (aloc, loc)
                        })
                        .collect();

                    let locals = Some(locals);
                    let assignment = Some(assignment);

                    let info = cpsc411::Info {
                        locals,
                        assignment,
                        ..info
                        // undead_out,
                        // conflicts,
                    };

                    self::P::module { info, tail }
                },
            }
        }

        let p = assign_p(p);
        Self { p }
    }

    /// UndeadAnalysis: Self/Locals -> Self/Undead
    ///
    /// ### Purpose:
    /// Performs undeadness analysis, decorating the program with undead-set
    /// tree. Only the info field of the program is modified.
    fn undead_analysis(self) -> Self {
        let Self { p } = self;

        fn undead_p(p: self::P) -> self::P {
            let ust = cpsc411::Tree::new();

            match p {
                self::P::module { info, tail } => {
                    let last = cpsc411::AlocSet::default();
                    let (cpsc411::Tree { nodes }, _) =
                        undead_tail(&tail, ust, last);

                    let undead_out = nodes.get(0).unwrap().clone();
                    let undead_out = Some(undead_out);

                    let info = cpsc411::Info { undead_out, ..info };
                    self::P::module { info, tail }
                },
            }
        }

        fn undead_tail(
            tail: &self::Tail,
            mut ust: cpsc411::Tree,
            last: cpsc411::AlocSet,
        ) -> (cpsc411::Tree, cpsc411::AlocSet) {
            match tail {
                self::Tail::halt { triv } => {
                    let node = cpsc411::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);

                    let last = undead_triv(triv, last);
                    (ust, last)
                },
                self::Tail::begin { effects, tail } => {
                    let sub_ust = cpsc411::Tree::new();
                    let (sub_ust, last) = undead_tail(&tail, sub_ust, last);

                    let (sub_ust, last) =
                        undead_effects(effects, sub_ust, last);

                    let node = cpsc411::Node::tree { tree: sub_ust };
                    ust.push_on(node);

                    (ust, last)
                },
            }
        }

        fn undead_effects(
            effects: &Vec<self::Effect>,
            ust: cpsc411::Tree,
            last: cpsc411::AlocSet,
        ) -> (cpsc411::Tree, cpsc411::AlocSet) {
            effects.into_iter().rev().fold(
                (ust, last),
                |(curr_ust, curr_last), effect| {
                    undead_effect(effect, curr_ust, curr_last)
                },
            )
        }

        fn undead_effect(
            effect: &self::Effect,
            mut ust: cpsc411::Tree,
            mut last: cpsc411::AlocSet,
        ) -> (cpsc411::Tree, cpsc411::AlocSet) {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let node = cpsc411::Node::alocs {
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
                    let node = cpsc411::Node::alocs {
                        alocs: last.clone(),
                    };
                    ust.push_on(node);

                    last.remove(aloc);
                    let last = undead_triv(triv, last);

                    (ust, last)
                },
                self::Effect::begin { effects } => {
                    // let node = cpsc411::Node::alocs {
                    //     alocs: last.clone(),
                    // };
                    // ust.push_on(node);

                    let sub_ust = cpsc411::Tree::new();
                    let (sub_ust, last) =
                        undead_effects(effects, sub_ust, last);

                    let node = cpsc411::Node::tree { tree: sub_ust };
                    ust.push_on(node);

                    (ust, last)
                },
            }
        }

        fn undead_triv(
            triv: &self::Triv,
            mut last: cpsc411::AlocSet,
        ) -> cpsc411::AlocSet {
            match triv {
                self::Triv::int64 { .. } => last,
                self::Triv::aloc { aloc } => {
                    let aloc = aloc.clone();
                    last.insert(aloc);
                    last
                },
            }
        }

        let p = undead_p(p);
        Self { p }
    }

    /// ConfictAnalysis: Self/Undead -> Self/Conflicts
    ///
    /// ### Purpose:
    /// Decorates a program with its conflict graph.
    fn conflict_analysis(self) -> Self {
        let Self { p } = self;

        fn conf_p(p: self::P) -> self::P {
            match p {
                self::P::module {
                    info:
                        cpsc411::Info {
                            undead_out,
                            locals,
                            assignment,
                            ..
                        },
                    tail,
                } => {
                    let locals = locals.unwrap();
                    let undead_out = undead_out.unwrap();

                    let conflicts = cpsc411::Graph::new(&locals);
                    let conflicts = conf_tail(&tail, &undead_out, conflicts);

                    let locals = Some(locals);
                    let undead_out = Some(undead_out);
                    let conflicts = Some(conflicts);

                    let info = cpsc411::Info {
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
            ust: &cpsc411::Node,
            conflicts: cpsc411::Graph,
        ) -> cpsc411::Graph {
            match tail {
                self::Tail::halt { .. } => conflicts,
                self::Tail::begin { effects, tail } => {
                    let conflicts = conf_tail(&tail, ust, conflicts);
                    conf_effects(effects, ust, conflicts)
                },
            }
        }

        fn conf_effects(
            effects: &Vec<self::Effect>,
            ust: &cpsc411::Node,
            conflicts: cpsc411::Graph,
        ) -> cpsc411::Graph {
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
            ust: &cpsc411::Node,
            mut conflicts: cpsc411::Graph,
        ) -> cpsc411::Graph {
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
                self::Effect::begin { effects } => {
                    conf_effects(effects, ust, conflicts)
                },
            }
        }

        fn remove_triv_from_alocs(
            triv: &self::Triv,
            mut alocs: cpsc411::AlocSet,
        ) -> cpsc411::AlocSet {
            match triv {
                self::Triv::int64 { .. } => alocs,
                self::Triv::aloc { aloc } => {
                    alocs.remove(aloc);
                    alocs
                },
            }
        }

        let p = conf_p(p);
        Self { p }
    }

    /// AssignRegisters: Self/Conflicts -> Self/Assignments
    ///
    /// ### Purpose:
    /// Performs graph-colouring register allocation. The pass attempts to fit
    /// each of the abstract location declared in the locals set into a
    /// register, and if one cannot be found, assigns it a frame variable
    /// instead.
    fn assign_registers(self) -> Self {
        let Self { p } = self;

        fn get_assignable_registers_from_assignments(
            assignments: &cpsc411::Assignments<target::Loc>,
            current_assignable_registers: &HashSet<cpsc411::Reg>,
            _: usize,
        ) -> Vec<cpsc411::Reg> {
            let locs = assignments.values().collect::<HashSet<&target::Loc>>();

            let registers = current_assignable_registers
                .into_iter()
                .filter_map(|reg| {
                    let reg = *reg;
                    let loc = target::Loc::reg { reg };

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
            cpsc411::Graph { graph }: &cpsc411::Graph,
        ) -> cpsc411::Aloc {
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
                let registers = cpsc411::Reg::current_assignable_registers();
                let length = registers.len();

                (registers, length)
            };

            match p {
                self::P::module { info, tail } => {
                    let cpsc411::Info {
                        locals, conflicts, ..
                    } = info;

                    let locals = locals.unwrap();
                    let conflicts = conflicts.unwrap();
                    let assignment =
                        HashMap::<cpsc411::Aloc, target::Loc>::default();

                    let assignment = recursive_assign(
                        locals,
                        conflicts,
                        assignment,
                        &current_assignable_registers,
                        k,
                    );

                    let assignment = Some(assignment);

                    let info = cpsc411::Info {
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
            mut locals: cpsc411::AlocSet,
            mut conflicts: cpsc411::Graph,
            assignments: cpsc411::Assignments<target::Loc>,
            current_assignable_registers: &HashSet<cpsc411::Reg>,
            k: usize,
        ) -> cpsc411::Assignments<target::Loc> {
            let empty_locals = locals.is_empty();

            match empty_locals {
                true => assignments,
                false => {
                    let aloc = lowest_order_aloc(&conflicts);

                    locals.remove(&aloc);
                    conflicts.remove(&aloc);

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
                    .map(|reg| target::Loc::reg { reg: reg.clone() })
                    .unwrap_or(target::Loc::fvar {
                        fvar: cpsc411::Fvar::fresh(),
                    });

                    assignments.insert(aloc, loc);

                    assignments
                },
            }
        }

        let p = assign_p(p);
        Self { p }
    }

    /// ReplaceLocations: Self/Assignments -> NestedAsmLang
    ///
    /// ### Purpose:
    /// Compiles Asm-lang v2/assignments to Nested-asm-lang v2, replaced each
    /// abstract location with its assigned physical location from the
    /// assignment info field.
    fn replace_locations(self) -> target::NestedAsmLang {
        let Self { p } = self;

        fn replace_p(p: self::P) -> target::P {
            match p {
                self::P::module {
                    info: cpsc411::Info { assignment, .. },
                    tail,
                } => {
                    let assignment = assignment.unwrap();

                    let tail = Box::new(tail);
                    let tail = replace_tail(tail, &assignment);

                    target::P::tail { tail }
                },
            }
        }

        fn replace_tail(
            tail: Box<self::Tail>,
            assignment: &cpsc411::Assignments<target::Loc>,
        ) -> target::Tail {
            match *tail {
                self::Tail::halt { triv } => {
                    let triv = replace_triv(triv, assignment);
                    target::Tail::halt { triv }
                },
                self::Tail::begin { effects, tail } => {
                    let effects = replace_effects(effects, assignment);
                    let tail = replace_tail(tail, assignment);
                    let tail = Box::new(tail);

                    target::Tail::begin { effects, tail }
                },
            }
        }

        fn replace_effects(
            effects: Vec<self::Effect>,
            assignment: &cpsc411::Assignments<target::Loc>,
        ) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(|effect| replace_effect(effect, assignment))
                .collect::<Vec<_>>()
        }

        fn replace_effect(
            effect: self::Effect,
            assignment: &cpsc411::Assignments<target::Loc>,
        ) -> target::Effect {
            match effect {
                self::Effect::set_aloc_triv { aloc, triv } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    let triv = replace_triv(triv, assignment);

                    target::Effect::set_loc_triv { loc, triv }
                },
                self::Effect::set_aloc_binop_aloc_triv {
                    aloc,
                    binop,
                    triv,
                } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    let triv = replace_triv(triv, assignment);

                    target::Effect::set_loc_binop_triv { loc, binop, triv }
                },
                self::Effect::begin { effects } => {
                    let effects = replace_effects(effects, assignment);
                    target::Effect::begin { effects }
                },
            }
        }

        fn replace_triv(
            triv: self::Triv,
            assignment: &cpsc411::Assignments<target::Loc>,
        ) -> target::Triv {
            match triv {
                self::Triv::int64 { int64 } => target::Triv::int64 { int64 },
                self::Triv::aloc { aloc } => {
                    let loc =
                        assignment.get(&aloc).map(target::Loc::clone).unwrap();
                    target::Triv::loc { loc }
                },
            }
        }

        let p = replace_p(p);
        target::NestedAsmLang { p }
    }

    /// AssignHomes: AsmLang -> NestedAsmLang
    ///
    /// ### Purpose:
    /// Compiles Asm-lang v2 to Nested-asm-lang v2, replacing each abstract
    /// location with a physical location.
    pub fn assign_homes(self) -> target::NestedAsmLang {
        self.uncover_locals().assign_fvars().replace_locations()
    }

    /// AssignHomesOpt: AsmLang -> NestedAsmLang
    ///
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

/// Compile: AsmLang -> ParenX64
///
/// ### Purpose:
/// Compiles the AsmLang program into a ParenX64 program.
impl Compile for AsmLang {
    fn compile(
        self,
        opt_level: crate::OptLevels,
    ) -> crate::paren_x64::ParenX64 {
        match opt_level {
            crate::OptLevels::O1 => self
                .assign_homes()
                .flatten_begins()
                .patch_instructions()
                .implement_fvars(),

            crate::OptLevels::O2 | crate::OptLevels::O3 => self
                .assign_homes_opt()
                .flatten_begins()
                .patch_instructions()
                .implement_fvars(),
        }
    }
}
