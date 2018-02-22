extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

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
        let mut sout_str = String::new("");
        let mut serr_str = String::new("");
        let sout = Future::new(|a| -> sout_str += a);
        let serr = Future::new(|a| -> serr_str += a);
        let result = myapp(std::env::args().collect(), sout, serr);
        assert_eq!(0 , result.wait().unwrap()));
        assert(sout_str, "Hello world");
        
    }
}
