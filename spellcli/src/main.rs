use std::io;

mod math;

mod word_process;
use word_process::*;

fn main() {
    println!("Check word: ");

    let mut input = String::new();

    let result = io::stdin().read_line(&mut input);
    if let Err(e) = result{
        println!("read_line error: {}", e);
    }
    let word = first_word(&input).to_lowercase();

    for wordsim in check_against(&word, 5).into_iter(){
        println!(
            "The word you typed ('{word}') is {}% simmalar to the word '{}'", 
            wordsim.get_sim() * 100.0,
            wordsim.get_word_2(),
        )
    }

    //println!("First word you typed: {}", word);

    //get_word_defs(&word);
}
