use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut deps: Vec<(String, String)> = Vec::new();
    let mut completed: HashMap<String, bool> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(step) => {
                let (source, target) = get_source_and_target(step);
                deps.push((source, target));
            },
            Err(_e) => break,
        }
    }

    let mut to_complete: Vec<String> = Vec::new();
    for dep in &deps {
        let (s, t) = dep;
        if !to_complete.contains(s) {
            to_complete.push(s.to_string());
        }
        if !to_complete.contains(t) {
            to_complete.push(t.to_string());
        }
    }

    to_complete.sort();
    while to_complete.len() > 0 {
        for i in 0..to_complete.len() {
            if canDo(i, &deps, &to_complete) {
                print!("{}", to_complete[i]);
                to_complete.remove(i);
                break;
            }
        }
    }

    Ok(())
}

fn canDo(i: usize, deps: &Vec<(String, String)>, to_complete: &Vec<String>) -> bool {
    for dep in deps {
        let (s, t) = dep;
        if *t == to_complete[i] && to_complete.contains(s) {
            return false;
        }
    }
    return true;
}

fn get_source_and_target(step: String) -> (String, String) {
    let parts: Vec<&str> = step.split(" ").collect();
    (parts[1].to_string(), parts[7].to_string())
}