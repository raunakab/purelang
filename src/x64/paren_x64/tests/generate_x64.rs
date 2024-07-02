use serial_test::serial;

use crate::utils;
use crate::x64::paren_x64 as source;

#[test]
#[serial]
fn basic() {
    let actual = source::ParenX64(source::P::begin(vec![])).generate_x64();

    let expected: String = "".into();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn one_instruction() {
    let actual =
        source::ParenX64(source::P::begin(vec![source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(10),
        }]))
        .generate_x64();

    let expected: String = "\tmov rax, 10".into();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn many_instruction() {
    let actual = source::ParenX64(source::P::begin(vec![
        source::S::set_reg_triv {
            reg: utils::Reg::rax,
            triv: source::Triv::int64(10),
        },
        source::S::set_reg_triv {
            reg: utils::Reg::rbx,
            triv: source::Triv::int64(11),
        },
    ]))
    .generate_x64();

    let expected: String = "\tmov rax, 10
\tmov rbx, 11"
        .into();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn labeled_instruction() {
    let actual =
        source::ParenX64(source::P::begin(vec![source::S::with_label {
            label: utils::Label::new_with_name("main"),
            s: Box::new(source::S::set_reg_triv {
                reg: utils::Reg::rbx,
                triv: source::Triv::int64(11),
            }),
        }]))
        .generate_x64();

    let expected: String = "L.main.0:
\tmov rbx, 11"
        .into();

    utils::reset_all_indices();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn labeled_jump() {
    let main = utils::Label::new_with_name("main");

    let jumper = utils::Label::new_with_name("jumper");

    let actual = source::ParenX64(source::P::begin(vec![
        source::S::with_label {
            label: main,
            s: Box::new(source::S::set_reg_triv {
                reg: utils::Reg::rbx,
                triv: source::Triv::int64(11),
            }),
        },
        source::S::with_label {
            label: jumper.clone(),
            s: Box::new(source::S::set_reg_triv {
                reg: utils::Reg::rbx,
                triv: source::Triv::int64(11),
            }),
        },
        source::S::jump(source::Trg::label(jumper)),
        source::S::set_reg_loc {
            reg: utils::Reg::rax,
            loc: source::Loc::reg(utils::Reg::rbx),
        },
    ]))
    .generate_x64();

    utils::reset_all_indices();

    let expected: String = "L.main.0:
\tmov rbx, 11
L.jumper.1:
\tmov rbx, 11
\tjmp L.jumper.1
\tmov rax, rbx"
        .into();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn nop() {
    let actual =
        source::ParenX64(source::P::begin(vec![source::S::nop])).generate_x64();

    utils::reset_all_indices();

    let expected: String = "".into();

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn compared_jump() {
    let main = utils::Label::new_with_name("main");

    let jumper = utils::Label::new_with_name("jumper");

    let jumper2 = utils::Label::new_with_name("jumper");

    let finish = utils::Label::new_with_name("finish");

    let actual = source::ParenX64(source::P::begin(vec![
        source::S::with_label {
            label: main,
            s: Box::new(source::S::compare_reg_opand_jump_if {
                reg: utils::Reg::rax,
                opand: source::Opand::int64(10),
                relop: utils::Relop::gt,
                label: jumper2.clone(),
            }),
        },
        source::S::with_label {
            label: jumper,
            s: Box::new(source::S::set_reg_triv {
                reg: utils::Reg::rbx,
                triv: source::Triv::int64(11),
            }),
        },
        source::S::jump(source::Trg::label(finish.clone())),
        source::S::with_label {
            label: jumper2,
            s: Box::new(source::S::set_reg_triv {
                reg: utils::Reg::rbx,
                triv: source::Triv::int64(12),
            }),
        },
        source::S::with_label {
            label: finish,
            s: Box::new(source::S::set_reg_loc {
                reg: utils::Reg::rax,
                loc: source::Loc::reg(utils::Reg::rbx),
            }),
        },
    ]))
    .generate_x64();

    utils::reset_all_indices();

    let expected: String = "L.main.0:
\tcmp rax, 10
\tjg L.jumper.2
L.jumper.1:
\tmov rbx, 11
\tjmp L.finish.3
L.jumper.2:
\tmov rbx, 12
L.finish.3:
\tmov rax, rbx"
        .into();

    assert_eq!(actual, expected);
}
