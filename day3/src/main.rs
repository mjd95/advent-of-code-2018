use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut claim_map: Vec<Vec<i8>> = Vec::new();
    for i in 0..1001 {
        let mut col: Vec<i8> = Vec::new();
        for j in 0..1001 {
            col.push(0);
        }
        claim_map.push(col);
    }

    let mut claim_map = [[0; 1001]; 1001];
    for line in reader.lines() {
        match line {
            Ok(claim) => {
                let parts: Vec<&str> = claim.split(" ").collect();
                if parts.len() < 4 {
                    break;
                }
                let pos: Vec<&str> = parts[2].split(",").collect();
                let dim: Vec<&str> = parts[3].split("x").collect();
                let l = usize::from_str(pos[0]).unwrap();
                let t = usize::from_str(pos[1].trim_right_matches(":")).unwrap();
                let r = l + usize::from_str(dim[0]).unwrap();
                let b = t + usize::from_str(dim[1]).unwrap();
                for i in l..r {
                    for j in t..b {
                        claim_map[j][i] += 1;
                    }
                }
            },
            Err(_e) => break,
        }
    }

    let mut contested = 0;
    for i in 0..1001 {
        for j in 0..1001 {
            if claim_map[i][j] > 1 {
                contested += 1;
            }
        }
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    for line in reader.lines() {
        match line {
            Ok(claim) => {
                let parts: Vec<&str> = claim.split(" ").collect();
                if parts.len() < 4 {
                    break;
                }
                let pos: Vec<&str> = parts[2].split(",").collect();
                let dim: Vec<&str> = parts[3].split("x").collect();
                let l = usize::from_str(pos[0]).unwrap();
                let t = usize::from_str(pos[1].trim_right_matches(":")).unwrap();
                let r = l + usize::from_str(dim[0]).unwrap();
                let b = t + usize::from_str(dim[1]).unwrap();
                let mut is_contested = false;
                for i in l..r {
                    for j in t..b {
                        if claim_map[j][i] > 1 {
                            is_contested = true;
                        }
                    }
                }
                if !is_contested {
                    println!("{}", claim);
                    break;
                }
            },
            Err(_e) => break,
        }
    }

    Ok(())
}
