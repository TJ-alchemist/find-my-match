mod utils;
mod users;

fn main() {
     
    users::User::greeting();
    
    if users::User::is_registered() {
        let registered_user = users::User::validate_credentials();
        match registered_user.is_user {
            true => println!("\n---------- Successfully logged in! ----------"),
            false => panic!("Invalid credentials!")
        };
        users::User::select_matches(registered_user.user_name, registered_user.gender);

    } else {
        let user = users::User::ask_user_info();
        user.register();
    }

}
