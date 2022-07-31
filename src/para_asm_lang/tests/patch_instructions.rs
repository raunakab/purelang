use serial_test::serial;

use crate::cpsc411;
use crate::para_asm_lang as source;
use crate::paren_x64_fvars as target;

#[test]
#[serial]
fn nothing() {
    let actual =
        source::ParaAsmLang(source::P::begin(vec![])).patch_instructions();

    cpsc411::reset_all_indices();

    let expected =
        target::ParenX64Fvars(target::P::begin(vec![target::S::with_label {
            label: cpsc411::Label::halt_label(),
            s: Box::new(target::S::nop),
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn basic() {
    let actual =
        source::ParaAsmLang(source::P::begin(vec![source::S::set_loc_triv {
            loc: source::Loc::reg(cpsc411::Reg::r10),
            triv: source::Triv::opand(source::Opand::int64(5)),
        }]))
        .patch_instructions();

    let expected = target::ParenX64Fvars(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: cpsc411::Reg::r10,
            triv: target::Triv::int64(5),
        },
        target::S::with_label {
            label: cpsc411::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn labeled_patch() {
    let label = cpsc411::Label::new_with_name("label");

    let actual =
        source::ParaAsmLang(source::P::begin(vec![source::S::set_loc_triv {
            loc: source::Loc::reg(cpsc411::Reg::r11),
            triv: source::Triv::label(label.clone()),
        }]))
        .patch_instructions();

    let expected = target::ParenX64Fvars(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: cpsc411::Reg::r11,
            triv: target::Triv::trg(target::Trg::label(label)),
        },
        target::S::with_label {
            label: cpsc411::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn patch_fvar_to_fvar_mov() {
    let fvar = cpsc411::Fvar::fresh();

    let actual =
        source::ParaAsmLang(source::P::begin(vec![source::S::set_loc_triv {
            loc: source::Loc::fvar(fvar.clone()),
            triv: source::Triv::opand(source::Opand::loc(source::Loc::fvar(
                fvar.clone(),
            ))),
        }]))
        .patch_instructions();

    let expected = target::ParenX64Fvars(target::P::begin(vec![
        target::S::set_reg_loc {
            reg: cpsc411::Reg::r10,
            loc: target::Loc::fvar(fvar.clone()),
        },
        target::S::set_fvar_trg {
            fvar,
            trg: target::Trg::reg(cpsc411::Reg::r10),
        },
        target::S::with_label {
            label: cpsc411::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}
