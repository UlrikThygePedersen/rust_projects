// use std::collections::Hashmap;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    // Give a text file
    let text = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    // Give string as a argument when cargo running
    // let text = &args[1];

    // Give a fixed text
    // let text = "Hello Prayson, how many words did you find?";

    let mut map = BTreeMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &map {
        println!("The word {word} was used {count} times!")
    }
    // println!("Words in the provided .txt file is: {:?}", map);
}
