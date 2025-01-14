use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

pub fn read_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

pub fn hasher(input: String) -> String {
    let mut output = DefaultHasher::new();
    input.hash(&mut output);
    output.finish().to_string()
}
