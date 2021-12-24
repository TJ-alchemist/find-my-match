use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub name: String,
    pub user_name: String,
    pub age: u8,
    pub gender: Gender,
    pub preferences: Vec<String>,
}

impl Users {

    pub fn greeting() {
        println!("\n|| Welcome to Find-My-Match");
        println!("|| Lets start.. (If you're not a user, please register and start again)\n");
    }

    pub fn is_registered() -> bool {
       println!("|| Are you already registered? [y/n]");
       let yes_or_no: String = utils::user_input("Something went wrong!");
       if yes_or_no == "y" {
           true
       } else {
           false
       }
    }

    pub fn ask_user_info() -> Self {
        println!("\n-> Enter your name:");
        let name: String = utils::user_input("Could not read name!");
        println!("\n-> Enter a user_name:");
        let user_name: String = utils::user_input("Could not read user_name!");
        println!("\n-> Enter your age:");
        let age: String = utils::user_input("Could not read age!");
        let age: u8 = age.trim().parse().expect("Could not parse age");
        println!("\n-> Enter gender (m/f):");
        let gender: String = utils::user_input("Could not read gender!");

        Self {
            name: name,
            user_name: user_name,
            age: age,
            gender: if gender == "m" {
                Gender::Male
            } else {
                Gender::Female
            },
            preferences: Vec::new()
        }
    }
    
    pub fn register(self){ 
        let mut all_users = utils::read_all("db.json").unwrap();
        all_users.push(self);
        let json_string = serde_json::to_string_pretty(&all_users).unwrap();
        utils::write_all(Path::new("db.json"), &json_string).unwrap();
        println!("** User registered! **");
    }

    pub fn validate_credentials() -> (bool, String, Gender) {
        let mut is_user: bool = false;
        let mut user_gender: Gender = Gender::Male;
        println!("\n|| Please enter your name and user_name");
        println!("\n-> Your name: [case-sensitive]");
        let name: String = utils::user_input("Could not read name");
        println!("\n-> Your user_name: [case-sensitive]");
        let user_name: String = utils::user_input("Could not read user_name");
        let all_users = utils::read_all("db.json").unwrap();
        for user in all_users {
            if user.name == name && user.user_name == user_name {
                is_user = true;
                user_gender = user.gender;
                break;
            }
        }

        (is_user, user_name, user_gender)
    }

    pub fn select_matches(user_name: String, user_gender: Gender) {
        let mut liked_or_not: String;
        let mut choices: Vec<String> = Vec::new();
        println!("\n(Registered users):\n");
        let all_users = utils::read_all("db.json").unwrap();
        for user in all_users {
            if user_name != user.user_name && user_gender != user.gender {
                println!("--------------------");
                println!(
                    "> Name: {},\n> Username: @{},\n> Age: {},\n> Gender: {:?}\n",
                    user.name, user.user_name, user.age, user.gender
                );
                println!("| Do you like: {}? [y/n]", user.name);
                liked_or_not = utils::user_input("Something went wrong!");
                if liked_or_not == "y" {
                    choices.push(user.user_name);
                }
                liked_or_not.clear();
                println!("--------------------");
            }
        }
        println!("\n|| The list of user information has exhausted!");
        println!("\n|| All your preferences: {:?}", choices);

    }

} 
