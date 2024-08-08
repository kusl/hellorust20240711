mod my_math;
mod shadowing;
mod my_data_types;
mod greatest_common_divisor;
mod game_stats;
mod game_error;
mod game_history;

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io;
use reqwest::blocking::Client;
use etcetera::{choose_app_strategy, AppStrategyArgs, AppStrategy};
use serde::{Deserialize, Serialize};
use game_stats::GameStats;
use game_error::GameError;
use game_history::GameHistory;

#[derive(Serialize, Deserialize)]
struct Config {
    analytics_consent: bool,
}

fn main() -> Result<(), GameError> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "Kushal Hada".to_string(),
        app_name: "KusGuessingGame".to_string(),
    }).unwrap();

    let config_path = strategy.config_dir().join("config.json");
    let mut config = if config_path.exists() {
        let config_data = fs::read_to_string(&config_path).expect("Unable to read config file");
        serde_json::from_str(&config_data).expect("Unable to parse config file")
    } else {
        Config { analytics_consent: false }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--update-consent" {
        println!("Do you consent to analytics? (yes/no)");
        let mut consent = String::new();
        io::stdin().read_line(&mut consent).expect("Failed to read line");
        config.analytics_consent = matches!(consent.trim().to_lowercase().as_str(), "yes" | "y");
        let config_data = serde_json::to_string(&config).expect("Unable to serialize config");
        fs::create_dir_all(strategy.config_dir()).expect("Unable to create config directory");
        fs::write(&config_path, config_data).expect("Unable to write config file");
        println!("Consent updated successfully.");
        return Ok(());
    }

    play_guessing_game(config.analytics_consent)?;
    Ok(())
}

fn play_guessing_game(analytics_consent: bool) -> Result<(), GameError> {
    println!("Guess the number!");
    println!("Remember, you can update your consent by running this application with the --update-consent flag.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = Vec::new(); // Initialize attempts as a vector
    let mut guesses = Vec::new(); // Initialize guesses as a mutable vector

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

        attempts.push(guess); // Add each guess to attempts
        guesses.push(guess); // Add each guess to guesses

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if analytics_consent {
                    fetch_hello_world()?;
                }
                println!("Game Statistics:");
                println!("Attempts: {:?}", attempts);
                println!("Secret Number: {}", secret_number);
                println!("Guesses: {:?}", guesses);

                let game_stats = GameStats {
                    attempts,
                    secret_number,
                    guesses,
                };

                save_game_stats(&game_stats)?;

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

    Ok(())
}

fn save_game_stats(game_stats: &GameStats) -> Result<(), GameError> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "Kushal Hada".to_string(),
        app_name: "KusGuessingGame".to_string(),
    }).unwrap();

    let stats_path = strategy.data_dir().join("game_stats.json");

    let mut game_history = if stats_path.exists() {
        let stats_data = fs::read_to_string(&stats_path)?;
        serde_json::from_str(&stats_data).unwrap_or(GameHistory { games: Vec::new() })
    } else {
        GameHistory { games: Vec::new() }
    };

    game_history.games.push(game_stats.clone());

    let stats_data = serde_json::to_string(&game_history).expect("Unable to serialize game stats");
    fs::create_dir_all(strategy.data_dir()).expect("Unable to create data directory");
    fs::write(stats_path, stats_data)?;

    Ok(())
}


fn fetch_hello_world() -> Result<(), GameError> {
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
