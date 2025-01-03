// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::cmp;
use std::fs;
use std::io;
use std::io::Write;

const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let mut  num_incorrect_guesses: u32 = 5;
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
        // Uncomment for debugging:
        // Your code here! :)
        println!("Welcome to CS110L Hangman!");
        println!("The word so far is: ");
        println!("You have guessed the following letters: ");
        println!("You have {} guesses left ", num_incorrect_guesses);
        println!("Please guess a letter: {:?}", secret_word_chars.iter().next());
    loop {
        let mut guess = String::new();

        io::stdout()
        .flush()
        .expect("Error flushing stdout.");

        io::stdin()
        .read_line(&mut guess)
        .expect("");

        let guess = Some(guess.chars().next().unwrap());

        if let Some(letter) = guess {
            for c in &secret_word_chars { 
                    if *c == letter {
                        println!("The word so far is: {}", sequence_with_char(&secret_word.chars().next().unwrap(), &guess));
                        println!("You have guessed the following letters: {:?}", guess);
                        println!("You have {} guesses left ", num_incorrect_guesses);
                        println!("Please guess a letter: {:?}", secret_word_chars.iter().next());
                    } 
            }
            println!("Sorry, that letter is not in the word");
            num_incorrect_guesses -= 1;
            if num_incorrect_guesses < 1 {
                println!("Sorry, you ran out of guesses!");
                    break;
            }            
        }

        


        //     else if NUM_INCORRECT_GUESSES < 1 && secret_word != guess{
            // println!("Sorry, you ran out of guesses!");
            // break;
        // } else {
            // println!("Sorry, that letter is not in the word");
        // }




        // match guess.cmp(&secret_word_chars.iter().next().unwrap()) {
        //     cmp::Ordering::Less    => println!("Too small!"),
        //     cmp::Ordering::Greater => println!("Too big!"),
        //     cmp::Ordering::Equal   => {
        //         println!("Congratulations you guessed the secret word: {}", secret_word);
        //         break;
        //     }
        // }
    }
}

fn sequence_with_char(s: &char, c: &String) -> String {
    let mut result = "-".repeat(s.to_string().len());
    let mid= s.to_string().len() / 2;

    result.replace_range(mid..mid+1, &c.to_string());
    result.to_string()
}