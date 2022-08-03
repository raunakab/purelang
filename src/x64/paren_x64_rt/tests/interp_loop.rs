use serial_test::serial;

use crate::utils;
use crate::x64::paren_x64_rt as source;

#[test]
#[serial]
#[should_panic]
#[ignore = "Insignificant test. Will never occur. (No instructions to get; unwrap fails)."]
fn empty_program() {
    let program = source::ParenX64Rt(source::P::begin(vec![]));

    let _ = program.interp_loop();
}

#[test]
#[serial]
#[should_panic]
#[ignore = "Insignificant test. Will never occur. (Never sets `rax`)."]
fn single_halt() {
    let program =
        source::ParenX64Rt(source::P::begin(vec![source::S::jump_trg(
            source::Trg::pc_addr(1),
        )]));

    let _ = program.interp_loop();
}

#[test]
#[serial]
fn basic() {
    let program = source::ParenX64Rt(source::P::begin(vec![
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(10),
        },
        source::S::nop,
    ]));

    let result = program.interp_loop();

    assert_eq!(result, 10);
}

#[test]
#[serial]
fn jump_over_set() {
    let program = source::ParenX64Rt(source::P::begin(vec![
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(30),
        },
        source::S::jump_trg(source::Trg::pc_addr(3)),
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(10),
        },
        source::S::nop,
    ]));

    let result = program.interp_loop();

    assert_eq!(result, 30);
}

#[test]
#[serial]
fn jump_over_two_sets() {
    let program = source::ParenX64Rt(source::P::begin(vec![
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(30),
        },
        source::S::jump_trg(source::Trg::pc_addr(3)),
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(20),
        },
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(10),
        },
        source::S::nop,
    ]));

    let result = program.interp_loop();

    assert_eq!(result, 10);
}
