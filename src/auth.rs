use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
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

pub fn _argon2_hasher(input: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(input.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}
