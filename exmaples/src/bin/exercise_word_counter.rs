use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct WordCounter(BTreeMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(BTreeMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, c: u64) {
//        for (key, value) in self.0.iter() {
//            println!("{}: {}", key, value);
//        }

        println!("-------------------------------------\n");

        let filter = self.0.iter().filter(|e| e.1 > &c);
        for (k, v) in filter {
            println!(" great than {},  {}: {}", c, k, v);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = arguments[1].clone();
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");

        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }

    println!("display all");
    word_counter.display(0);

    println!("display great than 2");
    word_counter.display(2);
}