use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    // println!("Secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
