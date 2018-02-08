use std::io::{self, Write};
#[macro_use] extern crate text_io;
extern crate rpassword;
extern crate keyring;

pub fn login() -> String {
    print!("username: ");
    io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    println!("entered: {}", username);

    let password;
    let keyring = keyring::Keyring::new("flik-rs", &username);
    match keyring.get_password() {
        Ok(v) => println!("password from keyring: {:?}", v),
        Err(_e) => {
            password = rpassword::prompt_password_stdout("password: ").unwrap();
            keyring.set_password(&password).unwrap();
        },
    }

    String::from("login not implemented yet") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login(), "login not implemented yet")
    }
}
