//! Mod with differnet ways to prossesing words
//! Only words made of letters in the range \[a-z\] are guaranteed to have a correct result from
//! most functions

use reqwest;
use wordnik_list as word_lib;

/// Gets the definitaion(s) of a word
pub fn get_word_defs(word: &str) -> String {
    let dictionary_check = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let body = reqwest::blocking::get(dictionary_check).unwrap()
        .text().unwrap();

    println!("body = {body:?}");

    "".to_string()
}

/// Gets the first word in a `&str`
pub fn first_word(string: &str) -> String {
    let string = string.trim();

    let mut word = String::new();

    for c in string.chars(){
        if !c.is_whitespace() {
            word.push(c);
        } else {
            return word;
        }
    }
    word
}

use crate::math::Average;
use std::cmp::Ordering;

/// Stores how simmalar two words are 
pub struct WordSim<'s>{
    word_1: &'s str,
    word_2: &'s str,
    sim_amount: f32, // 0.0 = 0%, 1.0 = 100%
}

impl<'s> WordSim<'s>{
    pub fn new(word_1: &'s str, word_2: &'s str) -> Self{
        let mut average = Average::new();
        for i in 0..( word_1.len().max(word_2.len()) ) {
            let letters_match: bool;

            if let Some(letter1) = word_1.get(i..i+1) &&
                let Some(letter2) = word_2.get(i..i+1){

                letters_match = letter1.cmp(letter2) == Ordering::Equal;
            } else {
                letters_match = false;
            }

            average.add(f32::from(letters_match));
        }                

        Self{
            word_1,
            word_2,
            sim_amount: average.solve().expect("at least one input should have a len > 0"),
        }
    }

    /// Gets how simmalar two stored words are 
    pub fn get_sim(&self) -> f32 {
        self.sim_amount
    }

}


///// Returns the top ten most simmalar words to the input string
//pub fn check_against<'s>(string: &'s str) -> [WordSim<'s>; 10] {

//}

