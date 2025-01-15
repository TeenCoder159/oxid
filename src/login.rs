use crate::auth;
use std::fs;

pub fn login() {
    loop {
        //
        println!("\nEnter username:");

        let hashed_username = auth::bcrypt_hasher(auth::read_line(), 14);

        let password = match fs::read_to_string(hashed_username) {
            Ok(contents) => contents,

            Err(_error) => {
                println!("Invalid username, please try again");
                break;
            }
        };

        println!("Enter password:");

        let hashed_password = auth::bcrypt_hasher(auth::read_line(), 14);

        if hashed_password == password {
            println!("Login successful!");
            break;
        } else {
            println!("An error occurred, please try again!")
        }
    }
}
