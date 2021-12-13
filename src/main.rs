use rand::Rng
use std::env;

#[derive(Debug)]
enum GenderType {
  	Male,
  	Female,
	Other
}

#[derive(Debug)]
struct PersonDetails {
  	name: String,
  	gender_type: GenderType
}

fn search_match(person_details: PersonDetails){
	let males = ["John", "Nicolas", "Tom", "Jake", "Brandon", "Robert"];
	let females = ["Sarah", "Gal", "Angelina", "Jennifer", "Monica", "Stacy"];
	let rng = rand::thread_rng().gen_range(0..6);
	match person_details.gender_type {
		GenderType::Male => println!("{:?} has a match with {:?}", person_details.name, females[rng]),
		GenderType::Female => println!("{:?} has a match with {:?}", person_details.name, males[rng]),
		GenderType::Other => println!("We don't know what gender type this person is interested in"),
	}	
}

fn main(){
	let  cli_name: String = env::args().nth(1).expect("Please provide a name!");
	let  cli_gender: String = env::args().nth(2).expect("Please provide your gender");
	
	let cli_gender_mapped: GenderType = match cli_gender {
		"M".to_string() => GenderType::Male,
		"F".to_string() => GenderType::Female,
		"O".to_string() => GenderType::Other,
	};
  	let detail = PersonDetails {
		name: cli_name,
		gender_type: cli_gender_mapped,
  	};
  	search_match(detail);
}
