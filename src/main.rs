use std::io;
use std::fs::{File, OpenOptions, write};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::{Read, BufReader};
use serde_json;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Gender {
    Male,
  	Female
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonDetails {
  	name: String,
    user_name: String,
    age: u8,
  	gender: Gender,
    preferences: Vec<String>
}
    
fn file_writer<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fs::write(path, &contents)?;
    Ok(())
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<PersonDetails>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Vec<PersonDetails> = serde_json::from_reader(reader)?;
    Ok(u)
}

fn main()  {

    // let users: Vec<PersonDetails> = Vec::new();
    println!("Welcome to Find-My-Match");
    println!("Lets start..");

    println!("Are you already registered? [y/n]");
    let mut registered: String = String::new();
    io::stdin().read_line(&mut registered).expect("Something went wrong");
    registered = registered.trim().to_string();

    let mut all_users = read_user_from_file("db.json").unwrap();
    let s1: String = String::from("Hello");
    if registered == "n" {
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
            } else {
                Gender::Female
            },
            preferences: Vec::new()
        };
        // let j = serde_json::to_string(&user)?;    
        // let user_json = serde_json::to_string(&user).unwrap();
        // println!("{}", user_json);
        
        // user.register();
        // let u = read_user_from_file("db.json").unwrap();
        // echo(u, &Path::new("b.txt")).unwrap_or_else(|why| {
        //     println!("! {:?}", why.kind());
        // });

        // let mut all_users = read_user_from_file("db.json").unwrap();
        all_users.push(user);
        let j = serde_json::to_string_pretty(&all_users).unwrap();
        file_writer(Path::new("db.json"), &j); 
        println!("User registered!");
        
        // println!("{:?}", j);
        // println!("{:#?}", u);

         // println!("{:#?}", all_users);
         // Create a vector of all the people details structure
         // loop through it and output the info
         // Ask whether u like him or her (Preference)
         // Take age of the other person
         

         // println!("{:?}", user);
    } else {
        // To validate name and username
        let mut is_user: bool = false;
        let mut user_gender: &Gender = &Gender::Male;
        println!("\nPlease enter your name and user_name");
        let mut details: Vec<String> = vec!["".to_string(), "".to_string()];
        println!("\nYour name:");
        io::stdin().read_line(&mut details[0]).expect("Cannot read name");
        details[0] = details[0].trim().to_string();
        println!("\nYour user_name:");
        io::stdin().read_line(&mut details[1]).expect("Could not read user_name");
        details[1] = details[1].trim().to_string();

        for user in &all_users {
            if user.name == details[0] && user.user_name == details[1] {
                is_user = true;
                user_gender = &user.gender;
                break;
            } else {
                println!("\nName and user_name doesn't match!");
                break;
            }
        }

        if is_user {
            let mut liked_or_not: String = String::new();
            let mut choices: Vec<&String> = Vec::new();
            println!("\nRegistered users:\n");
            for user in &all_users {
                if details[1] != user.user_name && user_gender != &user.gender {
                    println!("Name: {},\nUsername: {},\nAge: {},\nGender: {:?}\n", 
                             user.name, user.user_name, user.age, user.gender);
                    println!("Do you like: {}? [y/n]", user.name);
                    io::stdin().read_line(&mut liked_or_not).expect("Something went wrong");
                    liked_or_not = liked_or_not.trim().to_string();
                    if liked_or_not == "y" {
                        choices.push(&user.user_name);
                    }
                    liked_or_not.clear();
                }
            }
            println!("\nThe list of user information has exhausted!\n");
            println!("\nAll your preferences:\n{:?}", choices);
        }
    }
}
