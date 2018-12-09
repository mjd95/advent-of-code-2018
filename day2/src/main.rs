use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1])?;
    let f = BufReader::new(&f);

    let mut words: Vec<String> = Vec::new();
    for line in f.lines() {
        match line {
            Ok(word) => words.push(word),
            Err(_e) => break,
        }
    }

    let len = words.len();
    for i in 0..len {
        for j in i+1..len {
            if differentBy1(&words[i], &words[j]) {
                println!("{}, {}", words[i], words[j]);
                return Ok(())
            }
        }
    }
    println!("{:?}", words);
    Ok(())
}

fn differentBy1(word1: &String, word2: &String) -> bool {
    let char_vec1: Vec<char> = word1.chars().collect();
    let char_vec2: Vec<char> = word2.chars().collect();
    let mut num_diff = 0;
    for i in 0..char_vec1.len() {
        if char_vec1[i] != char_vec2[i] {
            num_diff += 1
        }
    }
    return num_diff == 1
}

fn part1() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1])?;
    let f = BufReader::new(&f);

    // let mut arr: Vec<HashMap<char, i32>> = Vec::new();
    let mut two_count = 0;
    let mut three_count = 0;
    for line in f.lines() {
        match line {
            Ok(val) => {
                let mut hm: HashMap<char, i32> = HashMap::new();
                let char_vec: Vec<char> = val.chars().collect();
                for c in char_vec {
                    let count = hm.entry(c).or_insert(0);
                    *count +=1 ;
                }
                for (_key, value) in &hm {
                    if *value == 2 {
                        two_count += 1;
                        break
                    }
                }
                for (_key, value) in &hm {
                    if *value == 3 {
                        three_count += 1;
                        break
                    }
                }
                // arr.push(hm);
            },
            Err(_e) => {},
        }
    }
    println!("{}", two_count * three_count);
    Ok(())
}
