pub mod asm_pred_lang;

pub type Source = crate::register_allocation::asm_pred_lang::AsmPredLang;

pub type Target =
    crate::structured_control_flow::nested_asm_lang::NestedAsmLang;

pub fn compile(p: Source) -> Target {
    p.uncover_locals()
        .undead_analysis()
        .conflict_analysis()
        .assign_registers()
        .replace_locations()
}
