use serial_test::serial;

use crate::structured_control_flow::paren_x64_fvars as source;
use crate::utils;
use crate::x64::paren_x64 as target;

#[test]
#[serial]
fn empty() {
    let actual =
        source::ParenX64Fvars(source::P::begin(vec![])).implement_fvars();

    let expected = target::ParenX64(target::P::begin(vec![]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn basic() {
    let actual = source::ParenX64Fvars(source::P::begin(vec![
        source::S::set_reg_triv {
            reg: utils::Reg::r10,
            triv: source::Triv::int64(5),
        },
        source::S::set_reg_binop_reg_int32 {
            reg: utils::Reg::r10,
            binop: utils::Binop::plus,
            int32: 5,
        },
    ]))
    .implement_fvars();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: utils::Reg::r10,
            triv: target::Triv::int64(5),
        },
        target::S::set_reg_binop_reg_int32 {
            reg: utils::Reg::r10,
            binop: utils::Binop::plus,
            int32: 5,
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn one_fvar_conversion() {
    let actual = source::ParenX64Fvars(source::P::begin(vec![
        source::S::set_fvar_int32 {
            fvar: utils::Fvar::fresh(),
            int32: 5,
        },
    ]))
    .implement_fvars();

    utils::reset_all_indices();

    let expected =
        target::ParenX64(target::P::begin(vec![target::S::set_addr_int32 {
            addr: utils::Addr {
                fbp: utils::Reg::current_frame_base_pointer(),
                disp_offset: 0,
            },
            int32: 5,
        }]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn many_fvar_conversions() {
    let actual = source::ParenX64Fvars(source::P::begin(vec![
        source::S::set_fvar_int32 {
            fvar: utils::Fvar::fresh(),
            int32: 5,
        },
        source::S::set_fvar_int32 {
            fvar: utils::Fvar::fresh(),
            int32: 6,
        },
    ]))
    .implement_fvars();

    utils::reset_all_indices();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_addr_int32 {
            addr: utils::Addr {
                fbp: utils::Reg::current_frame_base_pointer(),
                disp_offset: 0,
            },
            int32: 5,
        },
        target::S::set_addr_int32 {
            addr: utils::Addr {
                fbp: utils::Reg::current_frame_base_pointer(),
                disp_offset: 8,
            },
            int32: 6,
        },
    ]));

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn many_loc_conversions() {
    let actual = source::ParenX64Fvars(source::P::begin(vec![
        source::S::set_reg_loc {
            reg: utils::Reg::r10,
            loc: source::Loc::fvar(utils::Fvar::fresh()),
        },
        source::S::set_reg_binop_reg_loc {
            reg: utils::Reg::r11,
            binop: utils::Binop::multiply,
            loc: source::Loc::fvar(utils::Fvar::fresh()),
        },
    ]))
    .implement_fvars();

    utils::reset_all_indices();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_reg_loc {
            reg: utils::Reg::r10,
            loc: target::Loc::addr(utils::Addr {
                fbp: utils::Reg::current_frame_base_pointer(),
                disp_offset: 0,
            }),
        },
        target::S::set_reg_binop_reg_loc {
            reg: utils::Reg::r11,
            binop: utils::Binop::multiply,
            loc: target::Loc::addr(utils::Addr {
                fbp: utils::Reg::current_frame_base_pointer(),
                disp_offset: 8,
            }),
        },
    ]));

    assert_eq!(actual, expected);
}
