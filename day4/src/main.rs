use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

#[derive(Eq)]
struct Entry {
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
    action: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        if self.month != other.month {
            return self.month.cmp(&other.month)
        }
        if self.day != other.day {
            return self.day.cmp(&other.day)
        }
        if self.hour != other.hour {
            return self.hour.cmp(&other.hour)
        }
        return self.min.cmp(&other.min)
    }   
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.month == other.month && self.day==other.day && self.hour==other.hour && self.min==other.min
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut input: Vec<Entry> = Vec::new();
    let mut shifts: HashMap<usize, [i8; 60]> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(entry) => {
                let parts: Vec<&str> = entry.split(" ").collect();
                if parts.len() < 2 {
                    break
                }
                let date: Vec<&str> = parts[0].split("-").collect();
                let time: Vec<&str> = parts[1].split(":").collect();
                let mut action = 1;
                if parts[2] == "Guard" {
                    action = str::parse(parts[3].trim_left_matches("#")).unwrap();
                } else if parts[2] == "wakes" {
                    action = 0;
                }
                let entry = Entry{
                    month: str::parse(date[1]).unwrap(),
                    day: str::parse(date[2]).unwrap(),
                    hour: str::parse(time[0]).unwrap(),
                    min: str::parse(time[1].trim_right_matches("]")).unwrap(),
                    action: action,
                };
                input.push(entry);
            },
            Err(_e) => break,
        }
    }

    input.sort();

    let mut active_guard = 0;
    let mut falls_asleep = 0;
    let mut zero_vec = [0; 60];
    for e in input {
        println!("processing {}, {}, {}, {}, {}", e.month, e.day, e.hour, e.min, e.action);
        if e.action == 0 {
            if e.hour != 0 {
                panic!("oops");
            }
            match shifts.get_mut(&active_guard) {
                Some(arr) => {
                    for i in falls_asleep..e.min+1 {
                        arr[i] += 1;
                    }
                },
                None => println!("fucked"),
            }
        } else if e.action == 1 {
            if e.hour != 0 {
                panic!("oops");
            }
            falls_asleep = e.min;
        } else {
            active_guard = e.action;
            if !shifts.contains_key(&active_guard) {
                shifts.insert(active_guard, zero_vec.clone());
            }
        }
    }

    // find the guard with the most minutes asleep and their sleepiest min
    let mut sleepiest_guard = 0;
    let mut max_mins_sleep = 0;
    for (&k, &v) in shifts.iter() {
        let mut mins_sleep: i64 = 0;
        for i in 0..60 {
            mins_sleep += v[i] as i64;
        }
        if mins_sleep > max_mins_sleep {
            max_mins_sleep = mins_sleep;
            sleepiest_guard = k;
        }
    }

    let mut max_repeats = 0;
    let mut sleepiest_min = 0;
    for i in 0..60 {
        if shifts[&sleepiest_guard][i] > max_repeats {
            max_repeats = shifts[&sleepiest_guard][i];
            sleepiest_min = i;
        }
    }

    println!("sleepiest guard is {}", sleepiest_guard);
    println!("sleepiest min was {}", sleepiest_min);
    println!("product is {}", sleepiest_guard * sleepiest_min);

    // now for each minute, find the max # times any given guard slept on that minute.  what was that minute?
    let mut max_min = 0;
    let mut max_sleeps = 0;
    let mut guard = 0;
    for i in 0..60 {
        // find the max as we iterate over guards
        for (&k, &v) in shifts.iter() {
            if v[i] > max_sleeps {
                max_sleeps = v[i];
                max_min = i;    
                guard = k;
            }
        }
    }
    println!("most repeated min was {}", max_min);
    println!("the guard was {}", guard);
    println!("the product is {}", max_min * guard);
    Ok(())
}
