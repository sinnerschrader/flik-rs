extern crate rpassword;
extern crate rprompt;
extern crate keyring;

pub fn login() -> String {
    let username = rprompt::prompt_reply_stdout("username: ").unwrap();
    
    let keyring = keyring::Keyring::new("flik-rs", &username);
    let password = match keyring.get_password() {
        Ok(v) => v,
        Err(_e) => rpassword::prompt_password_stdout("password: ").unwrap(),
    };
    keyring.set_password(&password).unwrap();

    println!("{}/{}", username, password);
    String::from("login not implemented yet") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login(), "login not implemented yet")
    }
}
