use std::collections::HashMap;
use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut count = 0;
    let mut seen = HashMap::new();
    let mut should_continue = true;
    while should_continue {
        let f = File::open(&args[1])?;
        let f = BufReader::new(&f);
        for line in f.lines() {
            match line {
                Ok(val) => {
                    let parsed = val.parse::<i64>().unwrap();
                    count += parsed;
                    match seen.get(&count) {
                        Some(_val) => {
                            println!("first match is: {}", count);
                            should_continue = false;
                            break;
                        }
                        None => {},
                    }
                    seen.insert(count, true);
                },
                Err(_e) => println!("nothing left"),
            }
        }
    }
    Ok(())
}