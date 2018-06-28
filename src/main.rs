extern crate flik_lib;
extern crate reqwest;
extern crate rpassword;

use flik_lib::app;
use flik_lib::BaseService;

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let python_gil = BaseService::get_python_gil();
    let base_service = BaseService::new(&python_gil);
    base_service.login(
        &String::from(""),
        &String::from("uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq"),
    );

    // let client = reqwest::Client::new();
    // let res = client
    //     .post("https://blueant-uat.sinnerschrader.com/blueant/services/BaseService/Login")
    //     .form(&params)
    //     .build()
    //     .unwrap();

    // println!("Request: {:?}, {:?}", res, res.body());

    // let mut result = client.execute(res).unwrap();

    // println!("Result: {:?}", result.text().unwrap());

    //let sout = |a: &String| {
    //io::stdout().write(a.as_bytes()).unwrap();
    //io::stdout().flush().unwrap();
    //};

    //let serr = |a: &String| {
    //io::stderr().write(a.as_bytes()).unwrap();
    //};
    //let sin = |secured: bool| -> String {
    //if secured {
    //rpassword::read_password().unwrap()
    //} else {
    //let mut input: String = String::new();
    //io::stdin()
    //.read_line(&mut input)
    //.expect("reading stdin failed!");
    //input
    //}
    //};

    //let result = app(std::env::args().collect(), sin, &sout, serr);
    //sout(&String::from("\n"));
    //std::process::exit(result);
}
