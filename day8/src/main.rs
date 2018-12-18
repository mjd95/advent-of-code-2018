use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let mut reader = BufReader::new(&f);

    let mut line = String::new();
    let _len = reader.read_line(&mut line);
    let data: Vec<i64> = line.trim_right_matches("\n").split(" ").map(|x| str::parse(x).unwrap()).collect();
    // let data = vec![0, 1, 97];

    let mut meta_sum = 0;
    let mut val = 0;
    let (_, _, rmeta, rval) = process(&data, 0, &mut meta_sum, &mut val);

    println!("{}", rmeta);
    println!("{}", rval);

    Ok(())
}

fn process<'a>(data: &'a Vec<i64>, ptr: usize, meta_sum: &'a mut i64, val: &'a mut i64) -> (&'a Vec<i64>, usize, &'a mut i64, &'a mut i64) {
    if ptr >= data.len() {
        return (data, ptr, meta_sum, val)
    }

    let num_children = data[ptr] as usize;
    let num_metas = data[ptr+1] as usize;

    // process the children nodes
    let mut next_ptr = ptr+2;
    let mut child_vals = Vec::new();
    for _i in 0..num_children {
        let (_, t, _, child_val) = process(&data, next_ptr, meta_sum, val);
        child_vals.push(*child_val);
        next_ptr = t;
    }

    // process the meta for this node
    let mut cval = 0;
    if num_children == 0 {
        for i in next_ptr..next_ptr+num_metas {
            *meta_sum += data[i];
            cval += data[i];
            next_ptr += 1;
        }
    } else {
        for i in next_ptr..next_ptr+num_metas {
            *meta_sum += data[i];
            if 1 <= data[i] && data[i] <= (num_children as i64) {
                cval += child_vals[(data[i]-1) as usize];            
            }
            next_ptr += 1;
        }
    }

    // return, having consumed this node
    if num_children == 0 {
        *val = cval;
        return (data, next_ptr, meta_sum, val)
    } else {
        // this is computed in terms of the metadata and the vals of the child nodes
        *val = cval;
        return (data, next_ptr, meta_sum, val)
    }
}
