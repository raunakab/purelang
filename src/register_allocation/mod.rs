pub mod asm_pred_lang;

pub type Source = asm_pred_lang::AsmPredLang;

pub type Target = crate::structured_control_flow::Source;

pub fn compile(p: Source) -> Result<Target, String> {
    let p = p
        .uncover_locals()
        .undead_analysis()
        .conflict_analysis()
        .assign_registers()
        .replace_locations();
    Ok(p)
}
