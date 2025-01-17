use crate::auth;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use bcrypt::verify;
use std::fs;

pub fn _login_argon2(pwd_to_check: String, file_content: String) -> bool {
    let parsed_hash = PasswordHash::new(&file_content).expect("failed");
    Argon2::default()
        .verify_password(pwd_to_check.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn login() {
    loop {
        //
        println!("\nEnter username:");

        let path = format!("accounts/{}", auth::read_line());
        let saved_password = match fs::read_to_string(path) {
            Ok(contents) => contents,

            Err(_error) => {
                println!("Invalid username, please try again");
                continue;
            }
        };

        println!("Enter password:");

        let password = auth::read_line();

        match verify(password, &saved_password) {
            Ok(matched) if matched => {
                println!("Login successful!");
                break;
            }
            Ok(_) => println!("Wrong password, try again"),
            Err(e) => panic!("{e}"),
        }
    }
}
