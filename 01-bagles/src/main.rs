use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

const NUM_DIGITS: usize = 2;
const NUM_GUESSES: usize = 10;

fn main() -> io::Result<()> {
    print_instructions();

    loop {
        play_game();

        if !play_again() {
            break;
        }

        println!()
    }

    println!("\nThanks for playing!");
    Ok(())
}

fn print_instructions() {
    println!("Bagels, a deductive logic game.");
    println!("By Al Sweigart al@inventwithpython.com\n");
    println!("I am thinking of a {NUM_DIGITS}-digit number with no repeated digits.");
    println!("Try to guess what it is. Here are some clues.\n");
    println!("When I say:    That means:");
    println!("Pico           One digit is correct but in the wrong position.");
    println!("Fermi          One digit is correct and in the right position.");
    println!("Bagels         No digit is correct.\n");
    println!("For example, if the secret number was 248 and your guess was 843, the");
    println!("clues would be Fermi Pico.\n");
}

fn print_intro() {
    println!("I have thought of a number.");
    println!("  You have {NUM_GUESSES} guesses to get it.\n");
}

fn play_game() {
    let mut num_guesses: usize = 1;
    let secret_number = get_secret_number();

    print_intro();

    loop {
        println!("Guess #{}", num_guesses);

        let guess = get_guess();

        if guess == secret_number {
            println!("You got it!\n");
            break;
        }

        if num_guesses >= NUM_GUESSES {
            println!("\nYou ran out of guesses.");
            println!("The answer was {}.", secret_number);
            break;
        }

        let clues = get_clues(&guess, &secret_number);
        println!("{}", clues);

        num_guesses += 1;
        println!()
    }
}

fn play_again() -> bool {
    println!("Do you want to play again? (yes or no)");
    let input = read_stdin();

    input.to_lowercase().starts_with("y")
}

fn get_guess() -> String {
    loop {
        let input = read_stdin();

        if input.len() != NUM_DIGITS {
            println!("This is not {NUM_DIGITS} digits long.. try again!\n");
        } else if input.parse::<usize>().is_err() {
            println!("This is not a number.. try again!\n");
        } else {
            return input;
        }
    }
}

fn read_stdin() -> String {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed input read line.");

    input = input.trim().to_string();
    input
}

fn get_secret_number() -> String {
    let mut nums: Vec<char> = "0123456789".chars().collect();
    nums.shuffle(&mut thread_rng());

    nums[..NUM_DIGITS].iter().collect()
}

fn get_clues(guess: &str, secret_number: &str) -> String {
    let pairs = guess.chars().zip(secret_number.chars());
    let clues: Vec<String> = pairs
        .map(|(g, s)| {
            if g == s {
                return Some("Fermi".to_string());
            } else if secret_number.contains(g) {
                return Some("Pico".to_string());
            }

            return None;
        })
        .into_iter()
        .filter_map(|c| c)
        .collect();

    if clues.len() == 0 {
        return "Bagels".to_string();
    }

    clues.iter().sorted().join(" ")
}
