use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let mut reader = BufReader::new(&f);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(& mut buffer);

    let mut min_size = 1000000;
    for unit in 97..97+26+1 {
        let mut without = remove_unit(&buffer, unit);
        let size = react_polymer(&mut without);
        if size < min_size {
            min_size = size;
        }
    }

    println!("{}", min_size-1);

    Ok(())
}

fn remove_unit(full: &Vec<u8>, unit: u8) -> Vec<u8> {
    let mut reduced = Vec::new();
    for b in full {
        if *b != unit && *b != unit - 32 {
            reduced.push(*b);
        }
    }
    return reduced;
}

fn react_polymer(buffer: &mut Vec<u8>) -> usize {
    let mut size = buffer.len();
    let mut i = 0;
    while i < size-1 {
        if buffer[i]>buffer[i+1] && buffer[i]-buffer[i+1] == 32 {
            buffer.remove(i);
            buffer.remove(i);
            size -= 2;
            if i > 0 {
                i -= 1;
            }
        } else if buffer[i+1]>buffer[i] && buffer[i+1]-buffer[i] == 32 {
            buffer.remove(i);
            buffer.remove(i);
            size -= 2;
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    return size;
}
