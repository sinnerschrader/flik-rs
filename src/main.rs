#![feature(link_args)]
extern crate flik_lib;
extern crate libc;
extern crate rpassword;

use std::io::{self, Write};
use flik_lib::app;

use std::ffi::CString;
use std::os::raw::c_char;

mod binding;

// #[link_args = "-L/tmp/flik-rs/blueant-soap-cpp/Darwin"]
#[link(name = "blueant")]
extern "C" {
    fn newBlueantBase() -> *const libc::c_char;
    fn deleteBlueantBase(blueantBase: *const libc::c_char);
    fn blueantLogin(
        blueantBase: *const binding::soap,
        username: *const libc::c_char,
        password: *const libc::c_char,
        session: *const binding::_ns3__session,
    ) -> libc::c_int;
}
#[link(name = "gsoap")]
extern "C" {
    fn soap_new() -> binding::soap;
    fn soap_destroy(soap: *const binding::soap);
}

fn main() {
    let username = CString::new("").unwrap();
    let password = CString::new("uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq").unwrap();
    let mut blueantBase = unsafe { &mut soap_new() };
    let mut session = binding::_ns3__session {
        sessionID: &mut 0i8,
        personID: 0,
    };

    let mut v42: libc::c_int;
    {
        let session_ptr = unsafe { &mut session };

        v42 = unsafe {
            blueantLogin(
                blueantBase,
                username.as_ptr(),
                password.as_ptr(),
                session_ptr,
            )
        };
    }

    println!("Wurst {:?}, {:?}", v42, unsafe {
        CString::from_raw(session.sessionID)
    });

    unsafe {
        soap_destroy(blueantBase);
        binding::soap_end(blueantBase);
        binding::soap_free(blueantBase);
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
