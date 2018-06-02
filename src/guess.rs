// guess.rs
// Rohan Weeden
// Created: June 1, 2018


extern crate rand;

use guess::rand::Rng;
use std::io::Write;
use std::cmp::Ordering;
use std;


pub fn guess() {
    let num = rand::thread_rng().gen_range(1, 11);
    let num_guesses = 5;

    for i in 0..num_guesses {
        let guess = match get_guess() {
            Ok(num) => num,
            Err(_) => {
                println!("Thats not an integer...");
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Lol, higher"),
            Ordering::Greater => println!("Seriously? C'mon, lower"),
            Ordering::Equal => {
                println!("Thats it! Amazing...");
                return;
            }
        }
        println!("{} guesses remain", num_guesses - i - 1);
    }
    println!("Nope! Too slow!");
}


fn get_guess() -> Result<i64, std::num::ParseIntError> {
    print!("Enter an integer: ");
    std::io::stdout().flush()
        .expect("Flush failed");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess)
        .expect("Read failed");

    return guess.trim().parse::<i64>();
}
