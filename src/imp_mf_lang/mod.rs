pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::imp_cmf_lang as target;

pub struct ImpMfLang {
    pub p: self::P,
}

impl ImpMfLang {
    /// NormalizeBind: ImpMfLang -> ImpCmfLang
    ///
    /// ### Purpose:
    /// Compiles Imp-mf-lang v3 to Imp-cmf-lang v3, pushing set! under begin so
    /// that the right-hand-side of each set! is simple value-producing
    /// operation. This normalizes Imp-mf-lang v3 with respect to the
    /// equations.
    pub fn normalize_bind(self) -> target::ImpCmfLang {
        todo!()
    }
}
