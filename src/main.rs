extern crate clap;

mod lib;

use std::io::{self, Write};

use clap::{App, SubCommand};
use lib::flik;

fn main() {
    let sout = |a: &String| {
        io::stdout().write(a.as_bytes()).unwrap();
    };

    let serr = |a: &String| {
        io::stderr().write(a.as_bytes()).unwrap();
    };

    let result = myapp(std::env::args().collect(), &sout, serr);
    sout(&String::from("\n"));
    std::process::exit(result);
}

fn myapp<Out: Fn(&String), Err: Fn(&String)>(argv: Vec<String>, sout: Out, serr: Err) -> i32 {
    let matches = App::new("flik")
        .subcommand(SubCommand::with_name("hello"))
        .get_matches_from_safe(argv);

    match matches {
        Ok(val) => {
            let result = match val.subcommand_matches("hello") {
                Some(_) => flik("Hello"),
                _ => flik(""),
            };
            sout(&result);
            0
        }
        Err(message) => {
            serr(&message.message);
            1337
        }
    }
}
