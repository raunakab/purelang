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
                self::P::module {
                    tail,
                    info: cpsc411::Info { assignment, .. },
                } => {
                    let locals = uncover_tail(&tail);

                    let info = cpsc411::Info { locals, assignment };

                    self::P::module { info, tail }
                },
            }
        }

        fn uncover_tail(tail: &self::Tail) -> HashSet<cpsc411::Aloc> {
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

        fn uncover_effects(
            effects: &Vec<self::Effect>,
        ) -> HashSet<cpsc411::Aloc> {
            effects
                .iter()
                .map(uncover_effect)
                .flatten()
                .collect::<HashSet<_>>()
        }

        fn uncover_effect(effect: &self::Effect) -> HashSet<cpsc411::Aloc> {
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

        fn uncover_triv(triv: &self::Triv) -> HashSet<cpsc411::Aloc> {
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
                    info: cpsc411::Info { locals, .. },
                    tail,
                } => {
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
                        .collect::<HashMap<cpsc411::Aloc, target::Loc>>();

                    let info = cpsc411::Info { locals, assignment };

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
        todo!()
    }

    /// ConfictAnalysis: Self/Undead -> Self/Conflicts
    ///
    /// ### Purpose:
    /// Decorates a program with its conflict graph.
    fn conflict_analysis(self) -> Self {
        todo!()
    }

    /// AssignRegisters: Self/Conflicts -> Self/Assignments
    ///
    /// ### Purpose:
    /// Performs graph-colouring register allocation. The pass attempts to fit
    /// each of the abstract location declared in the locals set into a
    /// register, and if one cannot be found, assigns it a frame variable
    /// instead.
    fn assign_registers(self) -> Self {
        todo!()
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
                    let tail = Box::new(tail);
                    let tail = replace_tail(tail, &assignment);

                    target::P::tail { tail }
                },
            }
        }

        fn replace_tail(
            tail: Box<self::Tail>,
            assignment: &HashMap<cpsc411::Aloc, target::Loc>,
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
            assignment: &HashMap<cpsc411::Aloc, target::Loc>,
        ) -> Vec<target::Effect> {
            effects
                .into_iter()
                .map(|effect| replace_effect(effect, assignment))
                .collect::<Vec<_>>()
        }

        fn replace_effect(
            effect: self::Effect,
            assignment: &HashMap<cpsc411::Aloc, target::Loc>,
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
            assignment: &HashMap<cpsc411::Aloc, target::Loc>,
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
