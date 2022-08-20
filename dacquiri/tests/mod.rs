#![feature(generic_arg_infer)]

macro_rules! build_test {
    ($run_test:expr, $test_name:ident, $test_file_name:expr) => {
        #[test]
        fn $test_name() {
            let path_to_test = format!("tests/ui/{}", $test_file_name);
            $run_test(path_to_test)
        }
    }
}

macro_rules! compile_fail {
    ($test_name:ident) => {
        paste! {
            build_test!(
                |path: String| TestCases::new().compile_fail(path),
                [<invalid_ $test_name>],
                format!("fail/{}.rs", stringify!($test_name))
            );
        }
    }
}

macro_rules! compile_pass {
    ($test_name:ident) => {
        paste! {
            build_test!(
                |path: String| TestCases::new().pass(path),
                [<valid_ $test_name>],
                format!("pass/{}.rs", stringify!($test_name))
            );
        }
    }
}

// compile_fail!(fail_missing_check_simple for "fail_missing_check_simple.rs");

mod valid {
    use paste::paste;
    use trybuild::TestCases;

    compile_pass!(simple);
    compile_pass!(simple_guarded_function);
    compile_pass!(inherited_policy_explicit);
    compile_pass!(inherited_policy_implicit);
    compile_pass!(multistep_prove_impl_trait);
    compile_pass!(multistep_prove_impl_trait_implicit);
}

mod invalid {
    use paste::paste;
    use trybuild::TestCases;

    compile_fail!(simple);
    compile_fail!(simple_guarded_function);
}