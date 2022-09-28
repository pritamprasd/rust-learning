use rand::Rng;
use std::{cmp::Ordering, io};
// use rand::Rng. The Rng trait defines methods that random number generators implement,
// and this trait must be in scope for us to use those methods.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // gen_range method takes a range expression as an argument and generates a random number in the range

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        //A match expression is made up of arms. An arm consists of a pattern to match against,
        //and the code that should be run if the value given to match fits that arm’s pattern.
    }
}
