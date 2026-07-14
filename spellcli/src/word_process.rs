//! Mod with differnet ways to prossesing words
//! Only words made of letters in the range \[a-z\] are guaranteed to have a correct result from
//! most functions

use reqwest;
use wordnik_list as word_lib;
use serde_json::{self, Value};

/// Gets the definitaion(s) of a word
pub async fn get_word_defs(word: &str, num_of_defs: u32) -> Vec<WordDef> {
    let dictionary_check = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let body = reqwest::get(dictionary_check).await.unwrap()
        .text().await.unwrap();
    let json_body: Value = serde_json::from_str(&body)
        .expect("api.dictionaryapi.dev should always return valid json");

    let mut output = Vec::new();


    for def_num in 0..num_of_defs {
        let word_map = &json_body[0]["meanings"][def_num as usize];

        let word_def = &word_map["definitions"][0]["definition"];
        let part_of_speech = &word_map["partOfSpeech"];


        let word_def = word_def.as_str(); // might be somthing wrong with .as_str()
        let part_of_speech = part_of_speech.as_str();

        output.push(
            WordDef::new(
                word.to_string(), 
                if let Some(def) = word_def {
                    Some(def.to_string())
                } else { None },
                if let Some(pos) = part_of_speech {
                    Some(pos.to_string())
                } else { None },
            )
        );
    }
    output
}

/// Holds some definitions of a word
#[derive(Debug)]
pub struct WordDef{
    #[allow(unused)]
    pub word: String,
    pub def: Option<String>,
    pub part_of_speech: Option<String>,
}
impl WordDef {
    fn new(word: String, def: Option<String>, part_of_speech: Option<String>) -> Self {
        Self{
            word, 
            def, 
            part_of_speech,
        }
    }

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
/// Stores how similar two words are 
#[derive(Clone)]
pub struct WordSim<'s>{
    #[allow(unused)]
    word_1: &'s str,
    word_2: &'s str,
    sim_amount: f32, // 0.0 = 0%, 1.0 = 100%
}

use std::collections::HashMap;
impl<'s> WordSim<'s>{

    pub fn get_word_2(&self) -> &str {
        self.word_2
    }

    /// Calculates the similarity of of two words, if the words are the exact same, they have a
    /// similarity of `inf`
    pub fn new(word_1: &'s str, word_2: &'s str) -> Self{
        let mut average = Average::new();

        let mut word_1_iter = word_1.chars().into_iter();
        let mut word_2_iter = word_2.chars().into_iter();

        let largest_word_count = word_1.chars().count().max(word_2.chars().count()) as i32;

        // <letter, (word_1 count, word_2 count)>
        let mut word_map: HashMap<char, (i32, i32)> = HashMap::new();

        // getting the counts of each letter in the two words
        for _ in 0..largest_word_count{
            if let Some(letter_1) = word_1_iter.next() {
                (word_map.entry(letter_1).or_insert( (0, 0) )).0 += 1;
            }
            if let Some(letter_2) = word_2_iter.next() {
                (word_map.entry(letter_2).or_insert( (0, 0) )).1 += 1;
            }
        }

        // comparing the two word counts
        for (_letter, (count_1, count_2)) in word_map.into_iter() {
            let max = count_1.max(count_2);
            let min = count_1.min(count_2);
            let dif = max - min;

            average.add(1.0 - (dif as f32 / max as f32));
        }


        let mut sim_amount = average.solve().unwrap_or(0.0);
        // if the word is the exact same it is inf simmalar
        if sim_amount == 1.0 && word_1 == word_2 {
            sim_amount = 1.0/0.0;
        }


        Self{
            word_1,
            word_2,
            sim_amount,
        }
    }

    /// Gets how simmalar two stored words are 
    pub fn get_sim(&self) -> f32 {
        self.sim_amount
    }

}

/// Stores WordSims in order of sim
struct WordSimStorage<'s>{
    #[allow(unused)]
    len: u32,
    // stored [largest % .. smallest %]
    wordsims: Vec<WordSim<'s>>,
}

impl<'s> WordSimStorage<'s>{
    /// Creats a new WordSimStorage
    // wordsims will always be full
    fn new(size: u32) -> Self {
        Self{
            len: size,
            wordsims: vec![WordSim::new("", "");size as usize],
        }
    }

    fn add(&mut self, new_wordsim: WordSim<'s>) {

        let mut place_val = 0;
        for wordsim in self.wordsims.iter(){  
            if new_wordsim.get_sim() <= wordsim.get_sim() {
                break;
            } else {
                place_val += 1;
            }
        }

        if place_val == 0 { return; } // if less than all
        self.wordsims[(place_val-1) as usize] = new_wordsim;
    }

    fn get_vec(&self) -> Vec<WordSim<'s>> {
        self.wordsims.clone()
    }
}

/// Returns the top `top_number` most similar words to the input string
pub fn check_against<'s>(string: &'s str, top_number: u32) -> Vec<WordSim<'s>> {
    let all_words: Vec<&'static str> = word_lib::word_iterator().collect();

    let mut output: WordSimStorage<'s> = WordSimStorage::new(top_number);

    for word in all_words.into_iter() {
        output.add(WordSim::new(string, word));
    }

    output.get_vec()
}




