#![allow(non_camel_case_types)]

pub mod asm_lang;
pub mod block_asm_lang;
pub mod block_pred_lang;
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
