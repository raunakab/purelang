mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::imperative_abstractions::imp_cmf_lang as target;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct ProcImpCmfLang(pub self::P);

impl ProcImpCmfLang {
    /// ### Purpose:
    /// Compiles Proc-imp-cmf-lang v5 to Imp-cmf-lang v5 by imposing calling
    /// conventions on all calls and procedure definitions. The parameter
    /// registers are defined by the list current-parameter-registers.
    pub fn impose_calling_conventions(self) -> target::ImpCmfLang {
        todo!()
    }
}
