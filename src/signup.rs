use crate::auth;
use std::fs;
use zxcvbn::zxcvbn;
extern crate zxcvbn;

fn create_account(username: String, password: String) {
    match fs::write(username, auth::bcrypt_hasher(password, 14).to_string()) {
        Ok(_something) => println!("Account creation successful!"),
        Err(error) => panic!("{error}"),
    }
}

pub fn signup() {
    println!("Create an account:");
    loop {
        println!("Enter your email:");
        let username = auth::read_line();

        if !username.contains("@") || username.chars().count() < 7 {
            println!("Enter a valid email");
            continue;
        }

        loop {
            println!("Enter a Password:");

            let password = auth::read_line();
            match zxcvbn(&password, &[]) {
                Ok(entropy) => {
                    if entropy.score() >= 3 {
                        create_account(username, password);
                        return;
                    } else {
                        println!("Password isn't strong enough");
                        continue;
                    }
                }
                Err(e) => {
                    println!("Error: {e}");
                    continue;
                }
            }
        }
    }
}
