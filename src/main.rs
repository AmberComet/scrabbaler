

use std::{fs::File, io, collections::{btree_map, BTreeMap}, path::{self, Path}};
use regex::*;

fn main() {
    /*Create the BTres to sort the words into
    each they are sorted by value and within each value set they are then sorted by commonality
    this is only mallocing the memory 
    when these are implimented into the algorithm they should be implimened in this order*/
    let mut q_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut z_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut j_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut x_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut k_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut f_word:BTreeMap<i32, String> = BTreeMap::new(); //heh F words
    let mut w_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut v_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut h_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut y_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut b_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut m_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut g_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut p_word:BTreeMap<i32, String> = BTreeMap::new();
    let mut c_word:BTreeMap<i32, String> = BTreeMap::new();

    //this is where this misc words that only contain vowels and other verious 1pt letters go
    //as a result this tree should always be checked for straggalers
    let mut word_pool:BTreeMap<i32, String> = BTreeMap::new();



    println!("Intializing Word Bank");
    let words_from_file = words_initalization();


    /*this will take the vector of raw unsorted words and then sort them into the b trees to enable searching
    * when implimented the order that the trees are in and sorted into must be int the order they are malloced
    * when sorting into the trees also calculate the point value of each word and use that as key value
    * the reason is that it makes it easier to sort through the words
    * that way i can find words <= the total value of the letters then from thoes i can compaire the individual letters*/
    for word in words_from_file {
        
    }
    
}


fn words_initalization() -> Vec<String> {
    let file = File::open("Collins_Scrabble_Words_2019.txt").expect("Error: Cant Find File");
    let buf = io::BufReader::new(file);
    io::BufRead::lines(buf)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn word_value(word:&str) -> i32{
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
}