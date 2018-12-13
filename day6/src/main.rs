use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

#[derive(Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_i32(self.x * 100*self.y);
        state.finish();
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }    
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(&f);

    let mut points: Vec<Point> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(point) => {
                let coords: Vec<&str> = point.split(", ").collect();
                let x: i32 = str::parse(coords[0]).unwrap();
                let y: i32 = str::parse(coords[1]).unwrap();
                points.push(Point{x: x, y: y});
            },
            Err(_e) => break,
        }
    }

    // find min_x, max_x, min_y, max_y
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for point in & points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let mut closest_map: HashMap<& Point, i32> = HashMap::new();
    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {
            // find the closest point to (x, y)
            let cur_point = Point{x: x, y: y};
            match find_closest(& cur_point, & points) {
                Ok(pt) => {
                    *closest_map.entry(pt).or_insert(0) +=1;
                },
                Err(_e) => continue,
            }
        }
    }

    for y in min_y..max_y+1 {
        let cur_point = Point{x: min_x, y: y};
        match find_closest(& cur_point, & points) {
            Ok(pt) => {
                closest_map.remove(pt);
            },
            Err(_e) => continue,
        }

        let cur_point = Point{x: max_x, y: y};
        match find_closest(& cur_point, & points) {
            Ok(pt) => {
                closest_map.remove(pt);
            },
            Err(_e) => continue,
        }
    }

    for x in min_x..max_x+1 {
        let cur_point = Point{x: x, y: min_y};
        match find_closest(& cur_point, & points) {
            Ok(pt) => {
                closest_map.remove(pt);
            },
            Err(_e) => continue,
        }

        let cur_point = Point{x: x, y: max_y};
        match find_closest(& cur_point, & points) {
            Ok(pt) => {
                closest_map.remove(pt);
            },
            Err(_e) => continue,
        }
    }

    let mut max_found = std::i32::MIN;
    for (k, v) in &closest_map {
        if *v > max_found {
            max_found = *v;
        }
    }
    println!("max found is {}", max_found);

    let mut good_area_size = 0;
    let mut threshold = 10000;
    // for each point in the grid + some buffer, calculate the distance to all of the input points
    // if smaller than threshold, add to good area size
    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {
            let mut total_dist = 0;
            let cur_pt = Point{x: x, y: y};
            for pt in &points {
                total_dist += dist(&cur_pt, &pt);
                if total_dist >= threshold {
                    break
                }
            }
            if total_dist < threshold {
                good_area_size += 1;
            }
        }
    }

    println!("good area size is {}", good_area_size);

    Ok(())
}

fn find_closest<'a, 'b>(target: &'a Point, pts: &'b Vec<Point>) -> Result<&'b Point, &'static str> {
    let mut min_dist = std::i32::MAX;
    let mut count = 0;
    let mut closest = & Point{x: std::i32::MAX, y: std::i32::MAX};
    for pt in pts {
        let dist = dist(&pt, &target);
        if dist < min_dist {
            min_dist = dist;
            count = 0;
            closest = pt;
        } else if dist == min_dist {
            count += 1;
        }
    }
    if count > 0 {
        return Err("no closest point");
    }
    return Ok(closest);
}

fn dist(pt1: & Point, pt2: & Point) -> i32 {
    let mut xdist = pt1.x - pt2.x;
    if xdist<0 {
        xdist = -xdist;
    }
    let mut ydist = pt1.y - pt2.y;
    if ydist<0 {
        ydist = -ydist;
    }
    return xdist + ydist;
}