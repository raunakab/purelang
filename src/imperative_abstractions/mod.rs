pub mod imp_cmf_lang;
pub mod imp_mf_lang;
pub mod values_lang;
pub mod values_unique_lang;

pub type Source = crate::imperative_abstractions::values_lang::ValuesLang;

pub type Target = crate::register_allocation::asm_pred_lang::AsmPredLang;

pub fn compile(p: Source) -> Target {
    p.uniquify()
        .optimize_let_bindings()
        .sequentialize_let()
        .normalize_bind()
        .select_instructions()
}
