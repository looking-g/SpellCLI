use std::io;


mod math;

mod word_process;
use word_process::*;

use std::collections::HashMap;
use std::time;
use std::sync::{mpsc, Arc};
use std::thread;

fn main() {
    
    // input handleing 
    println!("Check word: ");

    let mut input = String::new();

    let result = io::stdin().read_line(&mut input);
    if let Err(e) = result{
        println!("read_line error: {}", e);
    }
    let timer = time::Instant::now(); // Timer for timing this CLI 
    let word = first_word(&input).to_lowercase();
    if word.len() < 2 { println!("input is too short"); return (); }


    // getting all the words
    let wordsims: Vec<WordSim> = check_against(&word, 5, 4).into_iter().rev().collect();

    // getting the word defs


    let (tx, rx) = mpsc::channel(); // transmitter and receiver for the def hashmap data
    let mut word_defs: HashMap<String, Vec<WordDef>> = HashMap::new();
    let client = Arc::new(reqwest::blocking::Client::new());
    for wordsim in wordsims.iter() {
        let (word_2, sender, client_ref) = (
            wordsim.get_word_2().to_string(),
            tx.clone(),
            client.clone(),
        );
        thread::spawn(move || {
            let def_data = (
                word_2.to_string(),
                get_word_defs(&word_2, 2_u32, &*client_ref),
            );
            sender.send(def_data).unwrap();
        });
    }
    std::mem::drop(tx);

    for def in rx{
        word_defs.insert(def.0, def.1);
    }


    // printing info
    for wordsim in wordsims.iter(){
        // this is (one of) the word(s) the code found to be simmalar to the input word
        let word2 = wordsim.get_word_2();

        let defs = word_defs.get(word2);

        if defs.is_none() {
            continue;
        }
        let defs = defs.unwrap(); // just checked if this none or not
                                  
        // checking that there are defs to print & creating the def text to be printed
        let mut def_text = String::new();
        let mut there_is_a_def = false; // if word2 has a def
        for def in defs.iter() {
            there_is_a_def |= def.def.is_some();

            if let Some(some_speech) = &def.part_of_speech{            
                if let Some(some_def) = &def.def{ 
                    def_text.push_str(
                        &format!("> {}: {}\n",
                            some_speech,
                            some_def,
                        )
                    );
                }
            }

        }
        if !there_is_a_def {
            continue;
        }
    

        println!(
            "'{word}' is {}% simmalar to the word '{}'\n{}", 
            wordsim.get_sim() * 100.0,
            wordsim.get_word_2(),
            def_text,
        );
        
    }

    println!("Took {} millis to run", timer.elapsed().as_millis());

}
