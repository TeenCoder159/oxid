// TODO: make it into a crate, and allow for custom strengths

mod auth;
mod login;
mod signup;

fn main() {
    // get username and password:
    println!("\nWhat would you like to do?\nLogin or Signup?");

    let user_choice = auth::read_line().to_lowercase();

    if user_choice == "signup" {
        signup::signup();
    } else {
        login::login();
    }
}
