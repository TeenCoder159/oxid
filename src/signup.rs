use crate::auth;
use std::fs;

fn create_account(username: String, password: String) {
    match fs::write(
        auth::bcrypt_hasher(username, 14),
        auth::bcrypt_hasher(password, 14),
    ) {
        Ok(_something) => println!("Account creation successful!"),
        Err(error) => panic!("{error}"),
    }
}

pub fn signup() {
    println!("Create an account:");
    loop {
        println!("Enter your email:");
        let username = auth::read_line();

        if !username.contains("@") && username.chars().count() < 7 {
            println!("Enter a valid email");
            continue;
        }

        loop {
            println!("Enter a Password:");

            let password = auth::read_line();

            if password.chars().count() < 8 || password.chars().count() > 20 {
                println!("Invalid password");
                continue;
            } else {
                let parsed: Result<i32, _> = password.clone().parse();
                match parsed {
                    Ok(_number) => {
                        create_account(username, password);
                        return;
                    }
                    Err(_) => println!("Password doesn't contain a number"),
                }
            }
        }
    }
}
