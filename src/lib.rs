#![allow(non_camel_case_types)]

macro_rules! make_begins {
    (($effects:expr, $end:expr) => $lang:ident::$atom:ident::$t:ident) => {{
        let length = $effects.len();
        match length {
            0 => $end,
            _ => {
                let t = if let $lang::$atom::begin {
                    effects: mut t_effects,
                    $t,
                } = $end
                {
                    t_effects.extend($effects);
                    $effects = t_effects;
                    $t
                } else {
                    Box::new($end)
                };

                $lang::$atom::begin {
                    effects: $effects,
                    $t: t,
                }
            },
        }
    }};
}

macro_rules! thrush {
    ($p:ident => $($phase:expr)=>*) => ({
        $(let $p = $phase($p)?;)*
        Ok($p)
    });
}

pub mod imperative_abstractions;
pub mod register_allocation;
pub mod structured_control_flow;
#[cfg(test)]
mod tests;
pub mod utils;
pub mod x64;

type Source = crate::imperative_abstractions::Source;

type Target = crate::x64::Target;

pub fn compile(p: Source) -> Result<Target, String> {
    thrush!(
        p
            => imperative_abstractions::compile
            => register_allocation::compile
            => structured_control_flow::compile
            => x64::compile
    )
}
