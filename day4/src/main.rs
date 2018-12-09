use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut shifts = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(entry) => {
                let active_guard: usize;
                let starts_sleeping: usize;
                let wakes_up: usize;
                let parts: Vec<&str> = entry.split(" ").collect();
                let time: Vec<&str> = parts[1].split(":").collect();

                let mins = time[1].trim_right_matches("]");

                if parts[2] == "Guard" {
                    active_guard = str::parse(parts[3].trim_left_matches("#")).unwrap();
                    if !shifts.contains_key(&active_guard) {
                        shifts.insert(active_guard, mut [0; 60]);
                    }
                } else if parts[3] == "falls" {
                    starts_sleeping = str::parse(mins).unwrap();
                } else if parts[3] == "wakes" {
                    wakes_up = str::parse(mins).unwrap();
                    for i in starts_sleeping..wakes_up {
                        shifts[&active_guard][i] += 1
                    }
                    // println!("{:?}", shifts[&active_guard])
                }
            },
            Err(_e) => break,
        }
    }
    Ok(())
}
