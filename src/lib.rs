extern crate rpassword;
extern crate rprompt;
extern crate keyring;

pub fn login() -> String {
    let username = rprompt::prompt_reply_stdout("username: ").unwrap();
    let password;

    let keyring = keyring::Keyring::new("flik-rs", &username);
    match keyring.get_password() {
        Ok(v) => password = v,
        Err(_e) => {
            password = rpassword::prompt_password_stdout("password: ").unwrap();
            keyring.set_password(&password).unwrap();
        },
    }

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
