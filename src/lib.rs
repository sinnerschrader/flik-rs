use std::io::{self, Write};
#[macro_use] extern crate text_io;
extern crate rpassword;

pub fn login() -> String {
    print!("username: ");
    io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    println!("entered: {}", username);

    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    println!("Your password is {}", pass);

    String::from("login not implemented yet") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login(), "login not implemented yet")
    }
}
