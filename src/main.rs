mod my_math;
mod shadowing;
mod my_data_types;
mod greatest_common_divisor;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
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

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Press Enter to exit...");
                let mut exit = String::new();
                io::stdin().read_line(&mut exit).expect("Failed to read line");
                break;
            }
        }

        let sum = my_math::add(guess as i32, secret_number as i32);
        println!("The sum of your guess and the secret number is: {sum}");
        let product = my_math::multiply(guess as i32, secret_number as i32);
        println!("The product of your guess and the secret number is: {product}");
    }
}