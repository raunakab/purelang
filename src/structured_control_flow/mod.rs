pub mod block_asm_lang;
pub mod block_pred_lang;
pub mod nested_asm_lang;
pub mod para_asm_lang;
pub mod paren_x64_fvars;

pub type Source =
    crate::structured_control_flow::nested_asm_lang::NestedAsmLang;

pub type Target = crate::x64::paren_x64::ParenX64;

pub fn compile(p: Source) -> Target {
    p.optimize_predicates()
        .expose_basic_blocks()
        .resolve_predicates()
        .flatten_program()
        .patch_instructions()
        .implement_fvars()
}
