extern crate assert_cli;
extern crate flik_lib;

#[cfg(test)]
mod integration {
    use assert_cli;
    use flik_lib::app;

    #[test]
    fn without_args() {
        assert_cli::Assert::main_binary()
            .stdout()
            .contains("Sorry, come again")
            .unwrap();
    }

    #[test]
    fn with_hello() {
        assert_cli::Assert::main_binary()
            .with_args(&["hello"])
            .stdout()
            .contains("Hello, world")
            .unwrap();
    }

    #[test]
    fn with_args_hello() {
        let mut sout_str = String::new();
        let mut serr_str = String::new();
        {
            let sout = |a: &String| {
                sout_str += a;
            };
            let serr = |a: &String| {
                serr_str += a;
            };

            let result = app(
                vec![String::from("flik"), String::from("hello")],
                sout,
                serr,
            );
            assert_eq!(0, result);
        }
        assert_eq!(sout_str, "Hello, world");
    }
}
