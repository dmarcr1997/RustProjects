use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_of_guess: u32 = 0;
    loop{
        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        num_of_guess += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("\n!!!!YOU WIN!!!!\n");
                break;
            }
        }
    }
    println!("Number of Guesses: {num_of_guess}");
}
