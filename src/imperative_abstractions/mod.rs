pub mod imp_cmf_lang;
pub mod imp_mf_lang;
pub mod proc_imp_cmf_lang;
#[cfg(test)]
mod tests;
pub mod values_lang;
pub mod values_unique_lang;

pub type Source = values_lang::ValuesLang;

pub type Target = crate::register_allocation::Source;

pub fn compile(p: Source) -> Result<Target, String> {
    p.check_values_lang().map(|p| {
        p.uniquify()
            .optimize_let_bindings()
            .sequentialize_let()
            .normalize_bind()
            .impose_calling_conventions()
            .select_instructions()
    })
}
