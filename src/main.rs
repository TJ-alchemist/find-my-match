use rand::Rng;
use std::env;
use std::collections::HashMap;
use std::io;


#[derive(Debug)]
enum Gender {
    Male,
  	Female,
	Other
}

//#[derive(Debug)]
//enum AgeGroup {
//    Youth,
//    Adults,
//    Seniors
//}
//
#[derive(Debug)]
struct PersonDetails {
  	name: String,
    user_name: String,
    age: u8,
  	gender: Gender,
//    preference: Option<HashMap<AgeGroup, Vec<PersonDetails>>>
}

//impl PersonDetails {
//    fn new_user(&self) -> Self {
//        let mut fields: u8 = 4;
//        while fields != 0 {
//            
//            io::stdin()
//                .read_line(&mut self.name)
//                .expect("Something went wrong");
//            fields -= 1;
//        }
//
//        Self {
//            name: self.name,
//            user_name: self.user_name,
//            age: self.age,
//            gender: self.gender,
//            preference: self.preference
//        }
//    }
//}

fn main() {
//    let users: Vec<PersonDetails> = Vec::new();
    println!("Welcome to Find-My-Match");
    println!("Lets start..");
    // Accepting name of the user
    println!("\nEnter your name:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Could not read name");
    name = name.trim().to_string();
    // Accepting user_name of the user
    println!("\nEnter a user_name:");
    let mut user_name: String = String::new();
    io::stdin().read_line(&mut user_name).expect("Could not read user_name");
    user_name = user_name.trim().to_string();
    // Accepting age of the user
    println!("\nEnter your age:");
    let mut age: String = String::new();
    io::stdin().read_line(&mut age).expect("Could not read age");
    let age: u8 = age.trim().parse().expect("Could not parse age");
    // Accepting gender of user
    println!("\nEnter gender (M/F/O):");
    let mut gender: String = String::new();
    io::stdin().read_line(&mut gender).expect("Could not read gender");
    gender = gender.trim().to_string();
    
    let user = PersonDetails {
        name: name,
        user_name: user_name,
        age: age,
        gender: if gender == "M" {
            Gender::Male
        } else if gender == "F" {
            Gender::Female
        } else {
            Gender::Other
        }
    };

    println!("{:?}", user);

}
