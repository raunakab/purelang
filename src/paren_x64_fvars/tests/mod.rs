use serial_test::serial;

use crate::cpsc411;
use crate::paren_x64 as target;
use crate::paren_x64_fvars as source;

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
            reg: cpsc411::Reg::r10,
            triv: source::Triv::int64(5),
        },
        source::S::set_reg_binop_reg_int32 {
            reg: cpsc411::Reg::r10,
            binop: cpsc411::Binop::plus,
            int32: 5,
        },
    ]))
    .implement_fvars();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_reg_triv {
            reg: cpsc411::Reg::r10,
            triv: target::Triv::int64(5),
        },
        target::S::set_reg_binop_reg_int32 {
            reg: cpsc411::Reg::r10,
            binop: cpsc411::Binop::plus,
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
            fvar: cpsc411::Fvar::fresh(),
            int32: 5,
        },
    ]))
    .implement_fvars();

    cpsc411::reset_all_indices();

    let expected =
        target::ParenX64(target::P::begin(vec![target::S::set_addr_int32 {
            addr: cpsc411::Addr {
                fbp: cpsc411::Reg::current_frame_base_pointer(),
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
            fvar: cpsc411::Fvar::fresh(),
            int32: 5,
        },
        source::S::set_fvar_int32 {
            fvar: cpsc411::Fvar::fresh(),
            int32: 6,
        },
    ]))
    .implement_fvars();

    cpsc411::reset_all_indices();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_addr_int32 {
            addr: cpsc411::Addr {
                fbp: cpsc411::Reg::current_frame_base_pointer(),
                disp_offset: 0,
            },
            int32: 5,
        },
        target::S::set_addr_int32 {
            addr: cpsc411::Addr {
                fbp: cpsc411::Reg::current_frame_base_pointer(),
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
            reg: cpsc411::Reg::r10,
            loc: source::Loc::fvar(cpsc411::Fvar::fresh()),
        },
        source::S::set_reg_binop_reg_loc {
            reg: cpsc411::Reg::r11,
            binop: cpsc411::Binop::multiply,
            loc: source::Loc::fvar(cpsc411::Fvar::fresh()),
        },
    ]))
    .implement_fvars();

    cpsc411::reset_all_indices();

    let expected = target::ParenX64(target::P::begin(vec![
        target::S::set_reg_loc {
            reg: cpsc411::Reg::r10,
            loc: target::Loc::addr(cpsc411::Addr {
                fbp: cpsc411::Reg::current_frame_base_pointer(),
                disp_offset: 0,
            }),
        },
        target::S::set_reg_binop_reg_loc {
            reg: cpsc411::Reg::r11,
            binop: cpsc411::Binop::multiply,
            loc: target::Loc::addr(cpsc411::Addr {
                fbp: cpsc411::Reg::current_frame_base_pointer(),
                disp_offset: 8,
            }),
        },
    ]));

    assert_eq!(actual, expected);
}
