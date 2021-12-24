mod utils;
mod users;

fn main() {
     
    users::Users::greeting();
    
    if users::Users::is_registered() {
        let details = users::Users::validate_credentials();
        match details.0 {
            true => println!("\n---------- Successfully logged in! ----------"),
            false => panic!("Invalid credentials!")
        };
        users::Users::select_matches(details.1, details.2);

    } else {
        let user = users::Users::ask_user_info();
        user.register();
    }

}
