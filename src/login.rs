use crate::auth;
use bcrypt::verify;
use std::fs;

pub fn login() {
    loop {
        //
        println!("\nEnter username:");

        let username = auth::read_line();

        let saved_password = match fs::read_to_string(username) {
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
