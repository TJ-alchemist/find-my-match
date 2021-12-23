use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

/*
 * Gender enum which holds 2 variants Male & Female. ..
 * These variants can be directly stored to a file. ..
 * These variants will be mapped according to the..
 * selection from terminal / prompt.
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

/*
 * PersonDetails is a struct which holds information ..
 * about the user.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonDetails {
    pub name: String,
    pub user_name: String,
    pub age: u8,
    pub gender: Gender,
    pub preferences: Vec<String>,
}

/*
 * Function for writing contents to a file. ..
 * Here its used for writing the details of the new users ..
 * to the file, which is of JSON format.
*/
pub fn file_writer<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fs::write(path, &contents)?;
    Ok(())
}

/*
 * Function to read all the contents from file. ..
 * Here its used for reading all the users' information.
*/
pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<PersonDetails>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Vec<PersonDetails> = serde_json::from_reader(reader)?;
    Ok(u)
}

/*
 * Function for accepting user input from terminal or ..
 * command prompt. The values are fetched in String type ..
 * and both leading and trailing spaces are removed using trim() ..
 * and again converted to String type
*/
pub fn user_input(error_msg: &str) -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(error_msg);
    input = input.trim().to_string();
    input
}
