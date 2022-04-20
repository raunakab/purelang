#![allow(non_camel_case_types)]

macro_rules! pass {
    ($NAME:ident, $SOURCE_LANG:ty, $TARGET_LANG:ty) => {
        impl $SOURCE_LANG {
            pub fn $NAME(self) -> $TARGET_LANG {
                self.into()
            }
        }
    };
}

pub mod asm_lang;
pub mod cpsc411;
pub mod imp_cmf_lang;
pub mod imp_mf_lang;
pub mod nested_asm_lang;
pub mod para_asm_lang;
pub mod paren_x64;
pub mod paren_x64_fvars;
#[cfg(test)]
mod tests;
pub mod values_lang;
pub mod values_unique_lang;

use asm_lang::AsmLang as SourceLang;
use paren_x64::ParenX64 as TargetLang;

/// PureLangCompile: SourceLang -> TargetLang
///
/// ### Purpose:
/// Compiles the source language into the final IL [intermediary language] (from
/// where this IL can be either interpreted or be used to generate X64 machine
/// code.
pub fn purelang_c(program: SourceLang) -> TargetLang {
    program
        .assign_homes_opt()
        .flatten_begins()
        .patch_instructions()
        .implement_fvars()
}
