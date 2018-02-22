extern crate clap;
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

mod lib;

use clap::{App, SubCommand}; 
use lib::flik;
use futures::stream::Stream;
use tokio_service::Service;
use tokio_proto::message::Message;
use tokio_proto::message::Body;


struct PrintStdout;

impl Service for PrintStdout {
    type Request = Message<String, Body<String, io::Error>>;
    type Response = Message<String, Body<String, io::Error>>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response,
                            Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let resp = Message::WithoutBody("Ok".to_string());

        match req {
            Message::WithoutBody(line) => {
                println!("{}", line);
                Box::new(future::done(Ok(resp)))
            }
            Message::WithBody(_, body) => {
                let resp = body
                    .for_each(|line| {
                        println!(" + {}", line);
                        Ok(())
                    })
                    .map(move |_| resp);

                Box::new(resp) as Self::Future
            }
        }
    }
}

fn main() {
    //let sout = Future::new(|a| -> stdout.write(a));
    let sout = PrintStdout; 
    //let serr = Future::new(|a| -> stderr.write(a));
    let serr = PrintStdout;
    let result = myapp(std::env::args().collect(), sout, serr);
    //os.exit(result.unwrap());
}

fn myapp(argv: Vec<String>, sout: Service, serr: Service) -> Box<Future<Item = u32>> {

    let mut ret = Box<Future<u32>>::new(0);
    let matches = App::new("flik")
        .subcommand(SubCommand::with_name("hello"))
        .get_matches_from_safe(argv);

    match matches {
        Ok(val) => { 
                        let result = match val.subcommand_matches("hello") {
                            Some(_) => { flik("Hello") },
                            _ =>  { flik("")}
                        }; 
                        sout(result); 
                        ret.resolve(0)
                    },
        Err(message) => {serr(message.message);
                            ret.resolve(1337)
                        }
        };
    return ret;
}
