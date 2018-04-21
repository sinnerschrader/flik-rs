extern crate flik_lib;
extern crate libc;
extern crate rpassword;

use std::io::{self, Write};
use flik_lib::app;

use std::ffi::CString;

mod baseServiceBinding;
mod worktimeAccountingServiceBinding;

#[link(name = "blueant")]
extern "C" {
    fn newBlueantBase() -> *mut baseServiceBinding::soap;
    fn deleteBlueantBase(blueant_base: *mut baseServiceBinding::soap);
}

fn main() {
    let username = CString::new("").unwrap();
    let password = CString::new("uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq").unwrap();
    let mut login_params = baseServiceBinding::_baseService3__LoginRequestParameter {
        username: username.into_raw(),
        password: password.into_raw(),
    };

    let blueant_base = unsafe { newBlueantBase() };
    let mut session = baseServiceBinding::_baseService3__session {
        sessionID: std::ptr::null_mut(),
        personID: 0,
    };

    let v42: std::os::raw::c_int;
    {
        v42 = unsafe {
            baseServiceBinding::soap_call___baseService1__Login(
                blueant_base,
                std::ptr::null(),
                std::ptr::null(),
                &mut login_params,
                &mut session,
            )
        };
    }

    println!("Wurst {:?}, {:?}", v42, unsafe {
        CString::from_raw(session.sessionID)
    });

    let log: std::os::raw::c_int = unsafe {
        baseServiceBinding::soap_call___baseService1__Logout(
            blueant_base,
            std::ptr::null(),
            std::ptr::null(),
            &mut baseServiceBinding::_baseService3__LogoutRequestParameter {
                sessionID: session.sessionID,
            },
            &mut baseServiceBinding::__baseService1__LogoutResponse { dummy: 0i8 },
        )
    };
    println!("Brot {:?}", log);

    unsafe {
        deleteBlueantBase(blueant_base);
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
