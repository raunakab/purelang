#![allow(non_camel_case_types)]

pub mod asm_lang;
pub mod cpsc411;
pub mod imp_cmf_lang;
pub mod imp_mf_lang;
pub mod nested_asm_lang;
pub mod para_asm_lang;
pub mod paren_x64;
pub mod paren_x64_fvars;
pub mod paren_x64_rt;
#[cfg(test)]
mod tests;
pub mod values_lang;
pub mod values_unique_lang;

pub enum OptLevels {
    O1,
    O2,
    O3,
}

impl Default for OptLevels {
    fn default() -> Self {
        Self::O3
    }
}

// use asm_lang::Values as SourceLang;
// use values_lang::ValuesLang as SourceLang;
// use paren_x64::ParenX64 as TargetLang;
// /// PureLangCompile: SourceLang -> TargetLang
// ///
// /// ### Purpose:
// /// Compiles the source language into the final IL [intermediary language]
// (from /// where this IL can be either interpreted or be used to generate X64
// machine /// code.
// pub fn purelang_c(program: SourceLang) -> TargetLang {
//     program
//         .uniquify()
//         .sequentialize_let()
//         .normalize_bind()
//         .select_instructions()
//         .assign_homes_opt()
//         .flatten_begins()
//         .patch_instructions()
//         .implement_fvars()
// }
