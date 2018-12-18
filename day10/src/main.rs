use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut pts: Vec<(i64, i64)> = Vec::new();
    let mut vels: Vec<(i64, i64)> = Vec::new(); 

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let parts: Vec<&str> = l.split(",").collect();
                let x :i64 = str::parse(parts[0]).unwrap();
                let y: i64 = str::parse(parts[1]).unwrap();
                let vx: i64 = str::parse(parts[2]).unwrap();
                let vy: i64 = str::parse(parts[3]).unwrap();
                pts.push((x, y));
                vels.push((vx, vy));
            },
            Err(_e) => break,
        }
    }

    // let mut prev = i64::max_value();
    // let mut i = 0;
    // while true {
    //     evolve(&mut pts, &vels);
    //     let (min_x, max_x, min_y, max_y) = bounding_box_dimension(&pts);
    //     let cur = max_x-min_x+max_y-min_y;
    //     println!("{}, {}, {}, {}, {}, {}", i, min_x, max_x, min_y, max_y, cur);
    //     if cur > prev {
    //         break;
    //     }
    //     prev = cur;
    //     i += 1;
    // }
    let (i_max, min_x, max_x, min_y, max_y, _) = (10813, 180, 248, 118, 137, 87); // results from commented out
    let mut i = 0;
    while i < i_max {
        evolve(&mut pts, &vels);
        i += 1;
    }
    print_board(&pts, min_x, max_x, min_y, max_y);

    Ok(())
}

fn bounding_box_dimension(pts: &Vec<(i64, i64)>) -> (i64, i64, i64, i64) {
    let mut min_x = i64::max_value();
    let mut max_x = i64::min_value();
    let mut min_y = i64::max_value();
    let mut max_y = i64::min_value();

    for (x, y) in pts {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    return (min_x, max_x, min_y, max_y);
}

fn evolve(pts: &mut Vec<(i64, i64)>, vels: &Vec<(i64, i64)>) {
    for i in 0..pts.len() {
        let (x, y) = pts[i];
        let (vx, vy) = vels[i];
        pts[i] = (x+vx, y+vy);
    }
}

fn print_board(board: &Vec<(i64, i64)>, min_x: i64, max_x: i64, min_y: i64, max_y: i64) {
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if board.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}