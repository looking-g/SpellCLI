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

    for string in ["maximum", "minimum", "total"].into_iter(){
        println!("The word you typed is {}% simmalar to the word '{}'", 
            WordSim::new(&word, string).get_sim() * 100.0, string);
    }

    //println!("First word you typed: {}", word);

    //get_word_defs(&word);
}
