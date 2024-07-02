pub mod block_asm_lang;
pub mod block_pred_lang;
pub mod nested_asm_lang;
pub mod para_asm_lang;
pub mod paren_x64_fvars;

pub type Source = nested_asm_lang::NestedAsmLang;

pub type Target = crate::x64::Source;

pub fn compile(p: Source) -> Result<Target, String> {
    let p = p
        .optimize_predicates()
        .expose_basic_blocks()
        .resolve_predicates()
        .flatten_program()
        .patch_instructions()
        .implement_fvars();
    Ok(p)
}
