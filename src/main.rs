// For now to disable the binding.rs warnings ...
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate flik_lib;
extern crate libc;
extern crate rpassword;

use std::io::{self, Write};
use flik_lib::app;

use std::ffi::CString;

mod binding;

#[link(name = "blueant")]
extern "C" {
    fn newBlueantBase() -> *mut binding::soap;
    fn deleteBlueantBase(blueantBase: *mut binding::soap);
}

fn main() {
    let username = CString::new("").unwrap();
    let password = CString::new("uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq").unwrap();
    let mut loginParams = binding::_baseService3__LoginRequestParameter {
        username: username.into_raw(),
        password: password.into_raw(),
    };

    let blueantBase = unsafe { newBlueantBase() };
    let mut session = binding::_baseService3__session {
        sessionID: std::ptr::null_mut(),
        personID: 0,
    };

    let v42: std::os::raw::c_int;
    {
        v42 = unsafe {
            binding::soap_call___baseService1__Login(
                blueantBase,
                std::ptr::null(),
                std::ptr::null(),
                &mut loginParams,
                &mut session,
            )
        };
    }

    println!("Wurst {:?}, {:?}", v42, unsafe {
        CString::from_raw(session.sessionID)
    });

    let log: std::os::raw::c_int = unsafe {
        binding::soap_call___baseService1__Logout(
            blueantBase,
            std::ptr::null(),
            std::ptr::null(),
            &mut binding::_baseService3__LogoutRequestParameter {
                sessionID: session.sessionID,
            },
            &mut binding::__baseService1__LogoutResponse { dummy: 0i8 },
        )
    };
    println!("Brot {:?}", log);

    unsafe {
        deleteBlueantBase(blueantBase);
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
