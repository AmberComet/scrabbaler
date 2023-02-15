use std::{fs::File, io};

fn main() {
    
}

fn words_initalization() -> Vec<String> {
    let file = File::open(Collins_Scrabble_Words_2019.txt).expect("Error: Cant Find File");
    let buf = io::BufReader::new(file);
    io::BufRead::lines(buf)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}