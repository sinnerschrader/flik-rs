pub fn flik(input: &str) -> String {
    match input {
        "Hello" => String::from("Hello, world"),
        _ => String::from("Sorry, come again")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(flik(""), "Sorry, come again")
    }

    #[test]
    fn test_hello() {
        assert_eq!(flik("Hello"), "Hello, world")
    }
}
