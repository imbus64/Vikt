use std::io;
use std::io::prelude::*;

/// Python like input method with prompt message
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");

    // Remove leading and trailing whitespaces and return
    input.trim().to_string()
}

/// Simple confirm dialogue
pub fn confirm(message: &str) -> bool {
    let answer = input(format!("{} [y/N]: ", message).as_str()).to_uppercase();
    if answer == "YES" || answer == "Y" {
        return true;
    }
    return false;
}
