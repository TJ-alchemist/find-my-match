mod utils;
mod users;


/// Flow:
/// Greeting the user (1).
/// Asking if user is registered or not (2).
/// Collecting user credentials (3).
/// Validating if user is registered (4).
/// Displaying list of all matches (5).
/// Asking new user for information (6).
/// Registering a new user by accepting information (7).

fn main() {
    users::User::greeting(); // (1)
    if users::User::is_registered() { // (2)
        let registered_user = users::User::validate_credentials(); // (3)
        match registered_user.is_user { // (5)
            true => println!("\n---------- Successfully logged in! ----------"),
            false => panic!("Invalid credentials!")
        };
        users::User::select_matches(registered_user.user_name, registered_user.gender); // (5)
    } else {
        let user = users::User::ask_user_info(); // (6)
        user.register(); // (7)
    }
}
