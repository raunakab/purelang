use serial_test::serial;

use crate::structured_control_flow::para_asm_lang as source;
use crate::structured_control_flow::paren_x64_fvars as target;
use crate::utils;

#[test]
#[serial]
fn nothing() {
    let actual =
        source::ParaAsmLang(source::P::begin(vec![])).patch_instructions();

    utils::reset_all_indices();

    let expected =
        target::ParenX64Fvars(target::P::begin(vec![target::S::with_label {
            label: utils::Label::halt_label(),
            s: Box::new(target::S::nop),
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn basic() {
    let actual =
        source::ParaAsmLang(source::P::begin(vec![source::S::set_loc_triv {
            loc: source::Loc::reg(utils::Reg::r10),
            triv: source::Triv::opand(source::Opand::int64(5)),
        }]))
        .patch_instructions();

    let expected = target::ParenX64Fvars(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: utils::Reg::r10,
            triv: target::Triv::int64(5),
        },
        target::S::with_label {
            label: utils::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn labeled_patch() {
    let label = utils::Label::new_with_name("label");

    let actual =
        source::ParaAsmLang(source::P::begin(vec![source::S::set_loc_triv {
            loc: source::Loc::reg(utils::Reg::r11),
            triv: source::Triv::label(label.clone()),
        }]))
        .patch_instructions();

    let expected = target::ParenX64Fvars(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: utils::Reg::r11,
            triv: target::Triv::trg(target::Trg::label(label)),
        },
        target::S::with_label {
            label: utils::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn patch_fvar_to_fvar_mov() {
    let fvar = utils::Fvar::fresh();

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
            reg: utils::Reg::r10,
            loc: target::Loc::fvar(fvar.clone()),
        },
        target::S::set_fvar_trg {
            fvar,
            trg: target::Trg::reg(utils::Reg::r10),
        },
        target::S::with_label {
            label: utils::Label::halt_label(),
            s: Box::new(target::S::nop),
        },
    ]));

    assert_eq!(actual, expected);
}
