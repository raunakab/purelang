pub mod paren_x64;
pub mod paren_x64_rt;

pub type Source = paren_x64::ParenX64;

pub type Target = String;

pub fn compile(p: Source) -> Result<Target, String> {
    let p = p.generate_x64();
    Ok(p)
}
