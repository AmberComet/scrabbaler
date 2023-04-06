use regex::*;
use std::{collections::BTreeSet, fs::File, io};

fn main() {
    /*Create the BTres to sort the words into
    each they are sorted by value and within each value set they are then sorted by commonality
    this is only mallocing the memory
    when these are implimented into the algorithm they should be implimened in this order
    Might switch back to Btree map if that might be faster*/
    let mut q_word: BTreeSet<String> = BTreeSet::new();
    let mut z_word: BTreeSet<String> = BTreeSet::new();
    let mut j_word: BTreeSet<String> = BTreeSet::new();
    let mut x_word: BTreeSet<String> = BTreeSet::new();
    let mut k_word: BTreeSet<String> = BTreeSet::new();
    let mut f_word: BTreeSet<String> = BTreeSet::new(); //heh F words
    let mut w_word: BTreeSet<String> = BTreeSet::new();
    let mut v_word: BTreeSet<String> = BTreeSet::new();
    let mut h_word: BTreeSet<String> = BTreeSet::new();
    let mut y_word: BTreeSet<String> = BTreeSet::new();
    let mut b_word: BTreeSet<String> = BTreeSet::new();
    let mut m_word: BTreeSet<String> = BTreeSet::new();
    let mut g_word: BTreeSet<String> = BTreeSet::new();
    let mut p_word: BTreeSet<String> = BTreeSet::new();
    let mut c_word: BTreeSet<String> = BTreeSet::new();

    //this is where this misc words that only contain vowels and other verious 1pt letters go
    //as a result this tree should always be checked for straggalers
    let mut word_pool: BTreeSet<String> = BTreeSet::new();

    let mut user_input: String = String::new();

    println!("Intializing Word Bank");
    let words_from_file = words_initalization();

    /*this will take the vector of raw unsorted words and then sort them into the b trees to enable searching
     * when implimented the order that the trees are in and sorted into must be int the order they are malloced
     * when sorting into the trees also calculate the point value of each word and use that as key value
     * the reason is that it makes it easier to sort through the words
     * that way i can find words <= the total value of the letters then from thoes i can compaire the individual letters*/
    for word in words_from_file {
        if word.contains('Q') {
            q_word.insert(word);
        } else if word.contains('Z') {
            z_word.insert(word);
        } else if word.contains('J') {
            j_word.insert(word);
        } else if word.contains('X') {
            x_word.insert(word);
        } else if word.contains('K') {
            k_word.insert(word);
        } else if word.contains('F') {
            f_word.insert(word);
        } else if word.contains('W') {
            w_word.insert(word);
        } else if word.contains('V') {
            v_word.insert(word);
        } else if word.contains('H') {
            h_word.insert(word);
        } else if word.contains('Y') {
            y_word.insert(word);
        } else if word.contains('B') {
            b_word.insert(word);
        } else if word.contains('M') {
            m_word.insert(word);
        } else if word.contains('G') {
            g_word.insert(word);
        } else if word.contains('P') {
            p_word.insert(word);
        } else if word.contains('C') {
            c_word.insert(word);
        } else {
            word_pool.insert(word);
        }
    }

    // This handles user input
    println!("plese enter what chars you have");

    io::stdin()
        .read_line(&mut user_input)
        .expect("There was an error reading line");
    let user_char: String = match user_input.trim().parse() {
        Ok(string) => string,
        Err(_) => panic!(),
    };

    let user_char = user_char.to_uppercase();

    let mut word_results: Vec<_> = Vec::new();

    //TODO Add multithreading here
    //this will iterate over each letter and only look at the Btrees that contain letters that the input contains
    if user_char.contains('Q') {
        word_results.append(&mut word_search(&user_char, &q_word));
    }
    if user_char.contains('Z') {
        word_results.append(&mut word_search(&user_char, &z_word));
    }
    if user_char.contains('X') {
        word_results.append(&mut word_search(&user_char, &x_word));
    }
    if user_char.contains('K') {
        word_results.append(&mut word_search(&user_char, &k_word));
    }
    if user_char.contains('F') {
        word_results.append(&mut word_search(&user_char, &f_word));
    }
    if user_char.contains('W') {
        word_results.append(&mut word_search(&user_char, &w_word));
    }
    if user_char.contains('V') {
        word_results.append(&mut word_search(&user_char, &v_word));
    }
    if user_char.contains('H') {
        word_results.append(&mut word_search(&user_char, &h_word));
    }
    if user_char.contains('Y') {
        word_results.append(&mut word_search(&user_char, &y_word));
    }
    if user_char.contains('B') {
        word_results.append(&mut word_search(&user_char, &b_word));
    }
    if user_char.contains('M') {
        word_results.append(&mut word_search(&user_char, &m_word));
    }
    if user_char.contains('G') {
        word_results.append(&mut word_search(&user_char, &g_word));
    }
    if user_char.contains('P') {
        word_results.append(&mut word_search(&user_char, &p_word));
    }
    if user_char.contains('C') {
        word_results.append(&mut word_search(&user_char, &c_word));
    }
    word_results.append(&mut word_search(&user_char, &word_pool));

    //Outputs the words
    for word in word_results {
        println!("{}", word)
    }
}

///Reads words from file and returns a vector of the words
fn words_initalization() -> Vec<String> {
    let file = File::open("Collins_Scrabble_Words_2019.txt").expect("Error: Cant Find File");
    let buf = io::BufReader::new(file);
    io::BufRead::lines(buf)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
///This function will take a String and Btree and will create regular expressions based on that string
/// It will then search through the B tree and apply the regular expression on each of thoes words
/// it will then return a vector of all matches
fn word_search(chars: &String, word_bank: &BTreeSet<String>) -> Vec<String> {
    let regex_chars = regex::escape(chars);

    let mut perms: Vec<String> = vec![String::from("")];
    for c in regex_chars.chars() {
        let mut new_perms = Vec::new();
        for perm in &perms {
            for i in 0..=perm.len() {
                let mut new_perm = perm.clone();
                new_perm.insert(i, c);
                new_perms.push(new_perm);
            }
        }
        perms = new_perms;
    }

    let mut regex_str = String::from("^(");
    for (i, perm) in perms.iter().enumerate() {
        if i > 0 {
            regex_str.push('|');
        }
        regex_str.push_str(&format!("{}[^{}]*$", perm, regex_chars));
    }
    regex_str.push(')');
    let regex = Regex::new(&regex_str).unwrap();

    word_bank
        .iter()
        .filter(|word| regex.is_match(word))
        .cloned()
        .collect()
}

/*fn word_value(word:&str) -> i32{
   let mut value=0;
    for c in word.chars(){
        if c=='Q'||c=='Z' {
            value+=10;
        }else if c=='J'|| c=='X' {
            value+=8;
        }else if c=='K' {
            value+=5;
        }else if c=='F'||c=='H'|| c=='V'||c=='W'||c=='Y' {
            value+=4;
        } else if c=='B'||c=='M'||c=='P'||c=='C' {
            value+=3;
        }else if c=='D'||c=='G' {
            value+=2;
        }else {
            value +=1;
        }
    }
    return value;
}*/
