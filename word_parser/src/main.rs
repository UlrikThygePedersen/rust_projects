use std::process;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid command, please run `word-counter <file-path>`");
        process::exit(1);
    }
    let file_path = &args[1];
    println!("Parsing the file {0}", file_path);
    let word_count = count_words_from(file_path);
    println!("{0}", word_count);
}

fn count_words_from(file_path: &str) -> u32 {
    let file = File::open(file_path).expect("error opening the file");
    let reader = BufReader::new(file);
    let mut word_count: u32 = 0;
    for line in reader.lines() {
        let curr: String = line.expect("error reading content of the file");
        let words: Vec<&str> = curr.split(' ').collect();
        let filtered_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 0).collect();
        word_count+= filtered_words.len() as u32
    }
    return word_count
}
