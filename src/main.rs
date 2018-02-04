extern crate clap;
mod lib;

use clap::{App, SubCommand}; 
use lib::flik;

fn main() {
    let matches = App::new("flik")
        .subcommand(SubCommand::with_name("hello"))
        .get_matches();

    let result = match matches.subcommand_matches("hello") {
        Some(_) => flik("Hello"),
        _ => flik("")
    };

    println!("{}", result);
}
