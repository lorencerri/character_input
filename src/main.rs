//! Author @truexpixels
//!
//! Solution to ["Rust Beginner Exercise: When will you turn 100?"](https://www.youtube.com/watch?v=JVrvzxGUMNY)
//! Originally from ["Character Input" Problem](http://www.practicepython.org/exercise/2014/01/29/01-character-input.html)
//!

extern crate chrono;

use std::io;
use chrono::prelude::*;

fn main() {

    // Request User Input
    let name = input("What is your username? ")
        .expect("Something went wrong!"); // Catch Errors
    println!("Hello, {}!", name); // Output User Input

    // Request User Input
    let age = input("What is your age? ")
        .expect("Failed to get age.") // Catch Errors
        .parse::<i32>().expect("Invalid Age."); // If unable to parse input as i32, catch error & output "Invalid Age."

    // Output User Input
    println!("You're {} years old!", age);

    // Request User Input
    let repeats = input("How many times should I output? ")
        .expect("Something went wrong!") // Catch Errors
        .parse::<u8>().expect("Invalid Amount."); // If unable to parse input as u8, catch error & output "Invalid Age."

    // Calculations
    let current_year = Utc::now().year();
    let hundred_year = 100 - age + current_year;

    // Output
    for _ in 0..repeats { // Repeat # of times
        if age > 100 { // If age is greater than 100, run this
            println!("Hey, {}! You turned turn 100 in the year: {}", name, hundred_year);
        } else { // If the previous statement didn't run, run this
            println!("Hey, {}! You'll turn 100 in the year: {}", name, hundred_year);
        }
    }

}

/// `input` mimics the input function in Python3
fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write; // Use Trait
    print!("{}", user_message); // Add message to print buffer
    io::stdout().flush()?; // Flush buffer (output everything in print)

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_right().to_owned())
}
