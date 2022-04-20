pub mod data;
#[cfg(test)]
mod tests;

pub use self::data::*;
use crate::imp_mf_lang as target;

pub struct ValuesUniqueLang {
    pub p: self::P,
}

impl ValuesUniqueLang {
    /// OptimizeLetBindings: ValuesUniqueLang -> ValuesUniqueLang
    ///
    /// ### Purpose:
    /// Optimizes let bindings by reordering them to minimize or maximize some
    /// metric.
    pub fn optimize_let_bindings(self) -> Self {
        self
    }

    /// SequentializeLet: ValuesUniqueLang -> ImpMfLang
    ///
    /// ### Purpose:
    /// Compiles Values-unique-lang v3 to Imp-mf-lang v3 by picking a particular
    /// order to implement let expressions using set!.
    pub fn sequentialize_let(self) -> target::ImpMfLang {
        todo!()
    }
}
