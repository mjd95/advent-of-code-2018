use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let num_gens: i64 = 50000000000;
    let mut state: Vec<i8> = Vec::new();

    let mut initial_state_chars = ".#..##..#.....######.....#....####.##.#.#...#...##.#...###..####.##.##.####..######......#..##.##.##".chars();
    // let mut initial_state_chars = "#..#.#..##......###...###".chars();
    let mut initial_state = parse_input(&mut initial_state_chars);
    state.append(&mut initial_state);

    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut rules: Vec<Vec<i8>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(rule) => {
                let parts: Vec<&str> = rule.split(" ").collect();
                if parts[2] == "#" {
                    let mut rule_chars = parts[0].chars();
                    let mut rule = parse_input(&mut rule_chars);
                    rules.push(rule);
                }
            },
            Err(_e) => break,
        }
    }

    for i in 0..state.len() {
        print!("{}", state[i]);
    }
    println!("");

    let mut left_shift = 0;
    let mut buffer_width = 0;
    let mut gen: i64 = 0;
    while gen < num_gens {
        let mut new_state: Vec<i8> = Vec::new();
        let mut padded_state = vec![0,0,0,0];
        padded_state.append(&mut state);
        padded_state.append(&mut vec![0, 0, 0, 0]);
        for i in 2..padded_state.len()-2 {
            if rules.contains(&padded_state[i-2..i+3].to_vec()) {
                new_state.push(1);
            } else {
                new_state.push(0);
            }
        }
        let mut first = 0;
        for i in 0..new_state.len() {
            if new_state[i] == 1 {
                first = i;
                left_shift += (first as i64) - 2;
                break;
            }
        }
        let mut last = 0;
        for i in 0..new_state.len() {
            if new_state[new_state.len()-1-i] == 1 {
                last = new_state.len()-i;
                break;
            }
        }
        state = new_state[first..last].to_vec();
        if gen%1000 == 0 {
            println!("completed gen {}, {}, {}", gen, first, last);
            let mut tot = 0;
            for i in 0..state.len() {
                if state[i] == 1 {
                    tot += (i as i64) + left_shift;
                }
            }
            println!("{}", tot);
        }
        gen += 1
    }

    let mut tot = 0;
    for i in 0..state.len() {
        if state[i] == 1 {
            tot += (i as i64) + left_shift;
        }
    }
    println!("{}", tot);

    Ok(())
}

fn parse_input(input: &mut std::str::Chars) -> Vec<i8> {
    let mut tr = Vec::new();
    while true {
        match input.nth(0) {
            Some(c) => {
                if c == "#".chars().nth(0).unwrap() {
                    tr.push(1);
                } else {
                    tr.push(0);
                }
            },
            None => break,
        }
    }
    return tr;
}