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

    let mut deps: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(step) => {
                let (source, target) = get_source_and_target(step);
                deps.push((source, target));
            },
            Err(_e) => break,
        }
    }

    let mut to_start: Vec<String> = Vec::new();
    for dep in &deps {
        let (s, t) = dep;
        if !to_start.contains(s) {
            to_start.push(s.to_string());
        }
        if !to_start.contains(t) {
            to_start.push(t.to_string());
        }
    }

    to_start.sort();

    let base_time = 60;
    let num_workers = 5;
    let mut times: HashMap<i64, i64> = HashMap::new();
    let mut tasks: HashMap<i64, String> = HashMap::new();
    for i in 0..num_workers {
        times.insert(i, 0);
        tasks.insert(i, "".to_string());
    }
    let mut tick = 0;

    let mut in_flight: Vec<String> = Vec::new();

    while to_start.len() > 0 || in_flight.len() > 0 {
        let mut i = 0;
        while i < to_start.len() {
            let mut delta = 1;
            if can_do(i, &deps, &to_start) {
                if not_blocked_by_inflight(to_start[i].clone(), &deps, &in_flight) {
                    for (k, v) in times.iter_mut() {
                        if *v == 0 {
                            *v = base_time + (to_start[i].as_bytes()[0] as i64) - 64;
                            tasks.insert(*k, to_start[i].clone());
                            in_flight.push(to_start[i].clone());
                            to_start.remove(i);
                            delta = 0;
                            break;
                        }
                    }
                }
            }
            i += delta;
        }

        for (k, v) in times.iter_mut() {
            if *v > 0 {
                *v -= 1;
            }

            if *v == 0 {
                for i in 0..in_flight.len() {
                    if in_flight[i] == *tasks.get(k).unwrap() {
                        in_flight.remove(i);
                        break;
                    }
                }
                print!("{}", tasks.get(k).unwrap());
                tasks.insert(*k, "".to_string());
            }
        }

        tick += 1;

    }

    println!("");
    println!("{}", tick);

    Ok(())
}

fn not_blocked_by_inflight(task: String, deps: &Vec<(String, String)>, in_flight: &Vec<String>) -> bool {
    for dep in deps {
        let (s, t) = dep;
        if *t == task && in_flight.contains(s) {
            return false;
        }
    }
    return true;
}

fn can_do(i: usize, deps: &Vec<(String, String)>, to_start: &Vec<String>) -> bool {
    for dep in deps {
        let (s, t) = dep;
        if *t == to_start[i] && to_start.contains(s) {
            return false;
        }
    }
    return true;
}

fn get_source_and_target(step: String) -> (String, String) {
    let parts: Vec<&str> = step.split(" ").collect();
    (parts[1].to_string(), parts[7].to_string())
}