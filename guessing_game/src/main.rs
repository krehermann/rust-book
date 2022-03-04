use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game :)");
    let target = rand::thread_rng().gen_range(1..101);

    loop {
        println!("What is your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //  .expect("Please type a number");
        println!("You guessed: {}", guess);

        match guess.cmp(&target) {
            Ordering::Greater => println!("Too big..."),
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
