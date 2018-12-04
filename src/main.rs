extern crate hamming;
extern crate regex;

use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod d1;
mod d2;
mod d3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a path to the datafile");
        return;
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", 
            display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", 
            display, why.description()),
        Ok(_) => (),
    }

    d3::solve(&s)
}