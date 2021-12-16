// use rand::Rng;
// use std::env;
// use std::collections::HashMap;
use std::io;
use std::fs::{File, OpenOptions, write};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::{Read, BufReader};
use serde_json;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct PersonDetails {
  	name: String,
    user_name: String,
    age: u8,
  	gender: Gender,
//    preference: Option<HashMap<AgeGroup, Vec<PersonDetails>>>
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn file_writer<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
// fn file_writer(v: Vec<PersonDetails>, path: &Path) -> io::Result<()> {
    //fs::write("foo.txt", b"Lorem ipsum")?;
    fs::write(path, &contents)?;
    Ok(())
}

// impl AsRef<User> for PersonDetails {
//     fn as_ref(&self) -> &User {
//         &self.user
//     }
// }

// serde library
impl PersonDetails {
    fn register(&self) {
        // let json_file_path = Path::new("db.json");
        // let file = File::open(json_file_path);
        // let reader = BufReader::new(file);
        // let users = serde_json::from_reader(reader);   
        // println!("{:?}", users);
    }
}

fn main()  {

    // let users: Vec<PersonDetails> = Vec::new();
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
    // let j = serde_json::to_string(&user)?;    
    // let user_json = serde_json::to_string(&user).unwrap();
    // println!("{}", user_json);
    
    // user.register();
    // let u = read_user_from_file("db.json").unwrap();
    // echo(u, &Path::new("b.txt")).unwrap_or_else(|why| {
    //     println!("! {:?}", why.kind());
    // });
    let all_users = vec![user];
    let j = serde_json::to_string_pretty(&all_users).unwrap();
    file_writer(Path::new("o.json"), &j);
    // println!("{:?}", j);
    // println!("{:#?}", u);

     // Create a vector of all the people details structure
     // loop through it and output the info
     // Ask whether u like him or her (Preference)
     // Take age of the other person
     

     // println!("{:?}", user);

}
