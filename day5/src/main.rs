use std::fs::File;
use std::io::{self, BufReader};
use std::env;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let mut reader = BufReader::new(&f);
    let mut buffer: Vec<u8> = Vec::new();
    let mut size = 0;
    let result = reader.read_to_end(& mut buffer);

    match result {
        Ok(res) => size=res,
        Err(_e) => println!("didn;t work"),
    }

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

    println!("{}", size-1);

    Ok(())
}
