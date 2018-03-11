extern crate flik_lib;
extern crate libc;
extern crate rpassword;

use std::io::{self, Write};
use flik_lib::app;
use libc::{c_char, c_int, c_long};

#[repr(C)]
struct soap;

#[repr(C)]
struct _ns3__LoginRequestParameter;

#[repr(C)]
struct _ns3__session;

#[link(name = "blueant")]
extern "C" {
    fn soap_call___ns1__Login(
        soap: *const soap,
        soap_endpoint: *const c_char,
        soap_action: *const c_char,
        ns3__LoginRequestParameter: *const _ns3__LoginRequestParameter,
        ns3__session: *mut _ns3__session,
    ) -> c_int;
}

fn main() {
    unsafe {
        // soap_call___ns1__Login((), ptr::null(), ptr:null(), (), ptr:null())
    }

    let sout = |a: &String| {
        io::stdout().write(a.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    };

    let serr = |a: &String| {
        io::stderr().write(a.as_bytes()).unwrap();
    };
    let sin = |secured: bool| -> String {
        if secured {
            rpassword::read_password().unwrap()
        } else {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("reading stdin failed!");
            input
        }
    };

    let result = app(std::env::args().collect(), sin, &sout, serr);
    sout(&String::from("\n"));
    std::process::exit(result);
}
