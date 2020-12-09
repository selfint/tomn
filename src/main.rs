mod keyword_consts;
mod tokenizer;
use std::env;
use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Checking file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let tokenizer = Tokenizer::new();

    for line in contents.lines() {
        for word in line.split_ascii_whitespace() {
            if let Ok(word_token) = tokenizer.convert_word_to_token(word) {
                println!("Word: {}, token: {:?}", word, word_token);
            } else {
                panic!("Unknown word: '{}', exiting!", word);
            }
        }
    }
}
