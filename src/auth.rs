use bcrypt::hash;
use std::io;

pub fn read_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

pub fn bcrypt_hasher(input: String, cost: u32) -> String {
    hash(input, cost).expect("Failed to hash password")
}
