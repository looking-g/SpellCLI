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

    // the difference here is for reduncency; api.dictionaryapi.dev & wordnik_list have differnt
    // words
    let mut num_print_def = 5_u32; // number of deffinitions that are prited
    let num_search_def = 10_u32; // number of deffinitions that are requested


    // getting all the words
    let wordsims: Vec<WordSim> = check_against(&word, num_search_def, 4).into_iter().rev().collect();


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
            // getting data
            let def_data = (
                word_2.to_string(),
                get_word_defs(&word_2, 2_u32, &*client_ref),
            );

            // only send the data if there is a def
            
            let mut there_is_a_def = false; 
            for def in def_data.1.iter() {
                there_is_a_def |= def.def.is_some();
            }

            if there_is_a_def { sender.send(def_data).unwrap(); }
        });
    }
    std::mem::drop(tx);

    for def in rx{
        word_defs.insert(def.0, def.1);
    }


    // printing info
    println!("");
    
    let mut wordsims_iter = wordsims.into_iter();

    match wordsims_iter.next() {
        Some(wordsim) => {
            if wordsim.get_sim() == 1.0/0.0 && 
                word_defs.get(wordsim.get_word_1()).is_some(){ 

                println!("✅ '{word}' IS a word ✅");
                println!("Definition of {word}");
                print_def(&wordsim, &word_defs, false);

                println!("Simmalar words:\n");

            } else {
                println!("❌ '{word}' is NOT a word ❌");
                println!("Simmalar words:\n");

                print_def(&wordsim, &word_defs, true); 
            }

        }
        None => {
            println!("No matches found for '{word}'");
            return;
        }
    }


    // printing other defs
    for wordsim in wordsims_iter{
        // this is (one of) the word(s) the code found to be simmalar to the input word

        print_def(&wordsim, &word_defs, true);

        num_print_def -= 1;

        if num_print_def == 0 { 
            break;
        }
        
    }

    if cfg!(debug_assertions) {
        println!("Took {} millis to run", timer.elapsed().as_millis());
    }

}

/// Prints out the definition and compareesent of a word if one is found
fn print_def(wordsim: &WordSim, lookup: &HashMap<String, Vec<WordDef>>, display_sim: bool) {

        let word = wordsim.get_word_1();
        let word2 = wordsim.get_word_2();

        let defs = lookup.get(word2);
        if defs.is_none() {
            return;
        }
        let defs = defs.unwrap(); // just checked if this none or not
                                  
        // checking that there are defs to print & creating the def text to be printed
        let mut def_text = String::new();
        for def in defs.iter() {
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
            
        if display_sim {
            def_text = format!(
                "'{word}' is {}% simmalar to the word '{}'\n{}", 
                wordsim.get_sim() * 100.0,
                wordsim.get_word_2(),
                def_text,
            );

        }

        println!( "{}", def_text);
}


