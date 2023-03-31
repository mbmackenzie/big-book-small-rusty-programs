use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use rand::Rng;
use std::cmp;
use std::io;
use std::io::Write;

const NUM_SIMS: usize = 100_000;
const INTRODUCTION: &str = "\
Birthday Paradox, by Al Sweigart al@inventwithpython.com

The birthday paradox shows us that in a group of N people, the odds
that two of them have matching birthdays is surprisingly large.
This program does a Monte Carlo simulation (that is, repeated random
simulations) to explore this concept.

(It's not actually a paradox, it's just a surprising result.)";

fn main() {
    println!("{}", INTRODUCTION);

    println!("\nHow many birthdays shall I generate? (Min 2, Max 100)");
    let num_bdays = get_num_bdays();
    let num_to_show = cmp::min(num_bdays, 5);

    if num_to_show != num_bdays {
        println!(
            "Here are {} birthdays (only showing {}):",
            num_bdays, num_to_show
        );
    } else {
        println!("Here are {} birthdays:", num_bdays)
    }

    let birthdays = get_birthdays(num_bdays);

    let birthday_string = birthdays[..num_to_show]
        .iter()
        .map(|d| d.format("%b %d").to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", birthday_string);

    let bday_match = get_match(birthdays);
    print!("In this simulation, ");
    match bday_match {
        Some(bday) => print!(
            "multiple people have birthdays on {}\n",
            bday.format("%b %d")
        ),
        None => print!("there are no matching birthdays.\n"),
    };

    println!(
        "\nGenerating {} random birthdays 100,000 times...",
        num_bdays
    );
    read_stdin(Some("Press enter to begin..."));
    println!();

    let mut sim_match: usize = 0;
    for i in 0..NUM_SIMS {
        if i % 10_000 == 0 {
            println!("{} simulations run...", i)
        }

        let birthdays = get_birthdays(num_bdays);
        let bday_match = get_match(birthdays);

        if bday_match.is_some() {
            sim_match += 1;
        }
    }

    println!("{} simulations run.\n", NUM_SIMS);

    let probability = 100.0 * (sim_match as f64 / NUM_SIMS as f64);
    println!("With {} simulations of {} people,", NUM_SIMS, num_bdays);
    println!("at least 1 birthday match occurred {} times -", sim_match);
    println!("making the probability or a match {:.2}%!\n", probability);
    println!("That's probably more than you would think!")
}

fn get_match(birthdays: Vec<NaiveDate>) -> Option<NaiveDate> {
    let unique_birthdays: Vec<NaiveDate> = birthdays
        .clone()
        .into_iter()
        .unique_by(|d| d.ordinal())
        .collect();

    if birthdays.len() == unique_birthdays.len() {
        return None;
    }

    for bday in birthdays.into_iter() {
        for unique_bday in unique_birthdays.iter() {
            if bday.ordinal() == unique_bday.ordinal() {
                return Some(bday);
            }
        }
    }

    panic!("Expected a duplicate but did not find one.")
}

fn get_birthdays(num_bdays: usize) -> Vec<NaiveDate> {
    let mut birthdays: Vec<NaiveDate> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..num_bdays {
        let year = 2022;
        let random_day = rng.gen_range(1..365);
        let date = NaiveDate::from_yo_opt(year, random_day).expect("Could not create date.");
        birthdays.push(date)
    }

    birthdays
}

fn read_stdin(msg: Option<&str>) -> String {
    match msg {
        Some(s) => print!("{}", s),
        None => print!("> "),
    }

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed input read line.");

    input = input.trim().to_string();
    input
}

fn get_num_bdays() -> usize {
    loop {
        let input = read_stdin(None);
        let n_bdays = match input.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("That is not a number.. Try again!");
                continue;
            }
        };

        if n_bdays < 2 || n_bdays > 100 {
            println!("That number is not between 2 and 100.. Try again!");
            continue;
        }

        return n_bdays;
    }
}
