mod my_math;
mod shadowing;
mod my_data_types;
mod greatest_common_divisor;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    play_guessing_game();
    fetch_hello_world()?;
    Ok(())
}

fn play_guessing_game() {
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

        let sum = my_math::add(guess, secret_number);
        println!("The sum of your guess and the secret number is: {sum}");
        let product = my_math::multiply(guess, secret_number);
        println!("The product of your guess and the secret number is: {product}");

        let gcd = greatest_common_divisor::gcd(guess, secret_number);
        println!("The greatest common divisor of your guess and the secret number is: {gcd}");
    }
}

fn fetch_hello_world() -> Result<(), Error> {
    let client = Client::new();
    match client.get("https://nice.runasp.net/Analytics/HelloWorld").send() {
        Ok(response) => {
            let text = response.text()?;
            println!("Response from server: {text}");
        }
        Err(e) => {
            println!("Failed to fetch response: {e}");
        }
    }
    Ok(())
}
