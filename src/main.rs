use std::io;
use std::path::Path;
use serde_json;
mod utils;


fn main()  {
    
    // CLUSTER: Introductory message
    // ----------
    println!("\n|| Welcome to Find-My-Match");
    println!("|| Lets start.. (If you're not a user, please register and start again)\n");
    // ----------
    
    // CLUSTER: Already registered or not
    // ----------
    println!("|| Are you already registered? [y/n]");
    let registered: String = utils::user_input("Something went wrong!");
    // ----------

    // CLUSTER: Reading database file of users [JSON]
    // ----------
    let mut all_users = utils::read_user_from_file("db.json").unwrap();
    // ----------
    
    // NOTE: Comparing different types (String with &str) [still works]
    if registered == "n" {
        
        // CLUSTER: Accepting user information
        // ----------
        println!("\n-> Enter your name:");
        let name: String = utils::user_input("Could not read name!"); 
        
        println!("\n-> Enter a user_name:");
        let user_name: String = utils::user_input("Could not read user_name!");    
        
        println!("\n-> Enter your age:");
        let age: String = utils::user_input("Could not read age!");
        let age: u8 = age.trim().parse().expect("Could not parse age");

        println!("\n-> Enter gender (m/f):");
        let gender: String = utils::user_input("Could not read gender!");
        // ----------
        
        // CLUSTER: Creating the user object
        // ----------
        let user = utils::PersonDetails {
            name: name,
            user_name: user_name,
            age: age,
            gender: if gender == "m" {
                utils::Gender::Male
            } else {
                utils::Gender::Female
            },
            preferences: Vec::new()
        };
        // ----------
        
        // CLUSTER: Adding user to the list of other users
        // ----------
        all_users.push(user);
        let j = serde_json::to_string_pretty(&all_users).unwrap();
        utils::file_writer(Path::new("db.json"), &j); 
        println!("** User registered! **");
        // ----------
        
    } else {
        
        // CLUSTER: To validate name and username
        // ----------
        let mut is_user: bool = false;
        let mut user_gender: &utils::Gender = &utils::Gender::Male;
        println!("\n|| Please enter your name and user_name");
        let mut details: Vec<String> = vec!["".to_string(), "".to_string()];
        println!("\n-> Your name:");
        io::stdin().read_line(&mut details[0]).expect("Cannot read name");
        details[0] = details[0].trim().to_string();
        println!("\n-> Your user_name:");
        io::stdin().read_line(&mut details[1]).expect("Could not read user_name");
        details[1] = details[1].trim().to_string();

        for user in &all_users {
            if user.name == details[0] && user.user_name == details[1] {
                is_user = true;
                user_gender = &user.gender;
                break;
            } else {
                println!("\n:: Name and user_name doesn't match!");
                break;
            }
        }
        // ----------

        // CLUSTER: To display all the profiles to the user
        // ----------
        if is_user {
            let mut liked_or_not: String = String::new();
            let mut choices: Vec<&String> = Vec::new();
            println!("\n(Registered users):\n");
            for user in &all_users {
                if details[1] != user.user_name && user_gender != &user.gender {
                    println!("--------------------");
                    println!("> Name: {},\n> Username: @{},\n> Age: {},\n> Gender: {:?}\n", 
                             user.name, user.user_name, user.age, user.gender);
                    println!("| Do you like: {}? [y/n]", user.name);
                    io::stdin().read_line(&mut liked_or_not).expect("Something went wrong");
                    liked_or_not = liked_or_not.trim().to_string();
                    if liked_or_not == "y" {
                        choices.push(&user.user_name);
                    }
                    liked_or_not.clear();  
                    println!("--------------------");
                }
            }
            println!("\n|| The list of user information has exhausted!");
            println!("\n|| All your preferences: {:?}", choices);
        }
        // ----------
    }
}
