use clap::{App, SubCommand};

pub fn app<Out: FnMut(&String), Err: FnMut(&String)>(
    argv: Vec<String>,
    mut sout: Out,
    mut serr: Err,
) -> i32 {
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

fn flik(input: &str) -> String {
    match input {
        "Hello" => String::from("Hello, world"),
        _ => String::from("Sorry, come again"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(flik(""), "Sorry, come again")
    }

    #[test]
    fn test_hello() {
        assert_eq!(flik("Hello"), "Hello, world")
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

    #[test]
    fn with_empty_args() {
        let mut sout_str = String::new();
        let mut serr_str = String::new();
        {
            let sout = |a: &String| {
                sout_str += a;
            };
            let serr = |a: &String| {
                serr_str += a;
            };

            let result = app(vec![String::from("flik")], sout, serr);
            assert_eq!(0, result);
        }
        assert_eq!(sout_str, "Sorry, come again");
    }
}
