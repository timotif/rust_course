use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!");
        // First create the mut varuable as a new empty String 
        let mut guess = String::new();
        // Construct a handle to the stdin
        // read_line to the reference of the guess var that needs to be mut
        // to be written on
        // expect prints the string in case of Panic
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");
        // Shadowing lets us reuse the guess variable name rather 
        // than forcing us to create two unique variables. To do so
        // the type of the new guess needs to be explicit
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");
        // Now I want to compare and handle the result of the comparison
        // I need match for that
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}