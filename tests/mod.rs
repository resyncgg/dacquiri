#![feature(generic_arg_infer)]

// mod ui;

macro_rules! build_test {
    ($run_test:expr, $test_name:ident, $test_file_name:literal) => {
        #[test]
        fn $test_name() {
            let path_to_test = format!("tests/ui/{}", $test_file_name);
            $run_test(path_to_test)
        }
    }
}

macro_rules! compile_fail {
    ($test_name:ident for $test_file_name:literal) => {
        build_test!(|path: String| trybuild::TestCases::new().compile_fail(path), $test_name, $test_file_name);
    }
}

// Some passing tests are ice'ing the compiler and I suspect it might have to do with this rustc bug
// https://github.com/rust-lang/rust/issues/92595
// So I'm going to disable the tests for right now
macro_rules! compile_pass {
    ($test_name:ident for $test_file_name:literal) => {
        //build_test!(|path: String| trybuild::TestCases::new().pass(path), $test_name, $test_file_name);
    }
}

compile_fail!(fail_missing_check_simple for "fail_missing_check_simple.rs");
compile_fail!(fail_missing_one_of_two_permissions for "fail_missing_one_of_two_permissions.rs");
compile_fail!(fail_elided_implicit_permission for "fail_elided_implicit_permission.rs");

compile_pass!(pass_simple_check for "pass_simple_check.rs");
compile_pass!(pass_two_permissions for "pass_two_permissions.rs");