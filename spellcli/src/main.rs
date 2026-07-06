use std::io;

fn main() {
    let mut input = String::new();

    let result = io::stdin().read_line(&mut input);
    if let Err(e) = result{
        println!("read_line error: {}", e);
    }

    println!("First word you typed: {}", first_word(&input));
}

fn first_word(string: &str) -> String {
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
