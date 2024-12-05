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

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    loop {
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("");

        println!("The word so far is: {}", sequence_with_char(&secret_word, &guess));
        println!("You have guessed the following letters: {}", guess);
        
        println!("Please guess a letter: {:?}", secret_word_chars.iter().next());

        println!("random word: {}", secret_word);

        if secret_word == guess {
            println!("You have {} guesses left ", NUM_INCORRECT_GUESSES-1);
        } else if NUM_INCORRECT_GUESSES < 1 && secret_word != guess{
            println!("Sorry, you ran out of guesses!");
            break;
        } else {
            println!("Sorry, that letter is not in the word");
        }

        let guess = guess.chars().next().unwrap();


        match guess.cmp(&secret_word_chars.iter().next().unwrap()) {
            cmp::Ordering::Less    => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal   => {
                println!("Congratulations you guessed the secret word: {}", secret_word);
                break;
            }
        }
    }
}

fn sequence_with_char(s: &str, c: &String) -> String {
    let mut result = "-".repeat(s.len());
    let mid    = s.len() / 2;

    result.replace_range(mid..mid+1, &c);
    result
}