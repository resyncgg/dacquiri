#![feature(generic_arg_infer)]

// mod ui;

#[test]
fn fail_missing_check_simple() {
    trybuild::TestCases::new().compile_fail("tests/ui/fail_missing_check_simple.rs");
}

#[test]
fn fail_missing_one_of_two_permissions() {
    trybuild::TestCases::new().compile_fail("tests/ui/fail_missing_one_of_two_permissions.rs");
}

#[test]
fn fail_elided_implicit_permission() {
    trybuild::TestCases::new().compile_fail("tests/ui/fail_elided_implicit_permission.rs");
}