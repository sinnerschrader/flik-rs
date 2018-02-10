extern crate keyring;
extern crate reqwest;
extern crate rpassword;
extern crate rprompt;
extern crate xmltree;

#[macro_use]
extern crate hyper;
header! { (SOAPAction, "SOAPAction") => [String] }
header! { (ContentType, "Content-Type") => [String] }
//use reqwest::header::ContentType;

use xmltree::Element;
pub fn login() -> String {
    let username = rprompt::prompt_reply_stdout("username: ").unwrap();

    let keyring = keyring::Keyring::new("flik-rs", &username);
    let password = match keyring.get_password() {
        Ok(v) => v,
        Err(_e) => rpassword::prompt_password_stdout("password: ").unwrap(),
    };
    keyring.set_password(&password).unwrap();

    println!("{}/{}", username, password);

    let login_req = include_str!("services/baseService/BaseBinding.Login.req.xml");

    let mut request_body = Element::parse(login_req.as_bytes()).unwrap();
    {
        let username = request_body
            .get_mut_child("Body")
            .unwrap()
            .get_mut_child("LoginRequestParameter")
            .unwrap()
            .get_mut_child("username")
            .unwrap();
        username.text = Some(<String>::from("max"));
    }
    {
        let password = request_body
            .get_mut_child("Body")
            .unwrap()
            .get_mut_child("LoginRequestParameter")
            .unwrap()
            .get_mut_child("password")
            .unwrap();
        password.text = Some(<String>::from("123456"));
    }

    let mut output = String::new();
    request_body.write(unsafe { output.as_mut_vec() }).unwrap();


    let client = reqwest::Client::new();
    let res = client
        .post("https://blueantasp36.proventis.net/demonew/services/BaseService/")
        .header(ContentType("application/soap+xml;charset=UTF-8".to_owned()))
        .header(SOAPAction("Login".to_owned()))
        .body(output)
        .send();
    // max/123456

    match res {
        Ok(mut v) => println!("{:?}", v.text()),
        Err(e) => println!("{:?}", e),
    }

    String::from("login not implemented yet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login(), "login not implemented yet")
    }
}
