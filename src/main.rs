extern crate flik_lib;

use std::io::{self, Write};
use flik_lib::app;

fn main() {
    let sout = |a: &String| {
        io::stdout().write(a.as_bytes()).unwrap();
    };

    let serr = |a: &String| {
        io::stderr().write(a.as_bytes()).unwrap();
    };
    let sin = || -> String {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("reading stdin failed!");
        input
    };

    let result = app(std::env::args().collect(),sin, &sout, serr);
    sout(&String::from("\n"));
    std::process::exit(result);
}
