use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use crate::users::User;


pub fn write_all<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fs::write(path, &contents)?;
    Ok(())
}

pub fn read_all<P>(path: P) -> Result<Vec<User>, Box<dyn Error>>
  where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Vec<User> = serde_json::from_reader(reader)?;
    Ok(u)
}

pub fn user_input(error_msg: &str) -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(error_msg);
    input = input.trim().to_string();
    input
}

