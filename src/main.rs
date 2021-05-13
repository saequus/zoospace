use crate::food::food::ToFeedAnimals;
mod food;
mod manage;
use std::env;


fn main () {
    let args: Vec<String> = env::args().collect(); 
    println!("Start of visit to Zoo Space");

    let mut animal_name = String::from("[Animal name not provided]");
    if args.len() > 1 {
        animal_name = String::from(&args[1]); 
    }
    let example_an  = food::food::Animal {
        animal_type: food::food::AnimalType::Horse,
        name: animal_name,
    };

    println!("Animal name: {}", example_an.show_name());

    println!("End of visit Zoo Space");
}



