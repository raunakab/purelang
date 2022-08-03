#![allow(non_camel_case_types)]

macro_rules! make_begins {
    (($effects:expr, $end:expr) => $lang:ident::$atom:ident::$t:ident) => ({
        let length = $effects.len();
        match length {
            0 => $end,
            _ => {
                let t = if let $lang::$atom::begin { effects: t_effects, $t } = $end {
                    $effects.extend(t_effects);
                    $t
                } else {
                    Box::new($end)
                };

                $lang::$atom::begin { effects: $effects, $t: t }
        },
        }
    })
}

pub mod utils;
pub mod imperative_abstractions;
pub mod register_allocation;
pub mod structured_control_flow;
pub mod x64;
#[cfg(test)]
mod tests;

type Source = crate::imperative_abstractions::values_lang::ValuesLang;

type Target = crate::x64::paren_x64::ParenX64;

pub fn compile(p: Source) -> Target {
    p.uniquify()
        .sequentialize_let()
        .normalize_bind()
        .select_instructions()
        .assign_homes_opt()
        .optimize_predicates()
        .expose_basic_blocks()
        .resolve_predicates()
        .flatten_program()
        .patch_instructions()
        .implement_fvars()
}
