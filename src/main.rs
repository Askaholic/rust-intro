extern crate rand;

use rand::Rng;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    let num = rand::thread_rng().gen_range(1, 11);

    loop {
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
                break;
            }
        }
    }
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
