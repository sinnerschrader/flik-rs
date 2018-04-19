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
    fn newBlueantBase() -> *mut binding::soap;
    fn deleteBlueantBase(blueantBase: *mut binding::soap);
    fn blueantLogin(
        blueantBase: *const binding::soap,
        username: *const libc::c_char,
        password: *const libc::c_char,
        session: *const binding::_ns3__session,
    ) -> libc::c_int;
}

fn main() {
    let mut username = CString::new("").unwrap();
    let mut password = CString::new("uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq").unwrap();
    let mut loginParams = binding::_ns3__LoginRequestParameter {
        username: username.into_raw(),
        password: password.into_raw()
    };

    let mut blueantBase = unsafe { newBlueantBase() };
    let mut session = binding::_ns3__session {
        sessionID: std::ptr::null_mut(),
        personID: 0,
    };

    let mut v42: std::os::raw::c_int;
    {
        let session_ptr = unsafe { &mut session };

        v42 = unsafe {
           binding::soap_call___ns1__Login(blueantBase, std::ptr::null(), std::ptr::null(), &mut loginParams, session_ptr)
        };
    }

    println!("Wurst {:?}, {:?}", v42, unsafe {
        CString::from_raw(session.sessionID)
    });

    let mut log: std::os::raw::c_int = unsafe 
    {
        binding::soap_call___ns1__Logout(blueantBase,
         std::ptr::null(),
         std::ptr::null(),
         &mut binding::_ns3__LogoutRequestParameter {sessionID: session.sessionID},
         &mut binding::__ns1__LogoutResponse {} )
    };

    println!("Brot {:?}", log);
    let mut log: std::os::raw::c_int = unsafe 
    {
        binding::soap_call___ns1__Logout(blueantBase,
         std::ptr::null(),
         std::ptr::null(),
         &mut binding::_ns3__LogoutRequestParameter {sessionID: std::ptr::null_mut()},
         &mut binding::__ns1__LogoutResponse {} )
    };
    println!("ZweitBrot {:?}", log);
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
