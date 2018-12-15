extern crate hamming;
extern crate regex;
extern crate chrono;

use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a path to the datafile");
        return;
    }

    let path = Path::new(&args[2]);
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

    match args[1].as_str() {
        "1" => d1::solve(&s),
        "2" => d2::solve(&s),
        "3" => d3::solve(&s),
        "4" => d4::solve(&s),
        "5" => d5::solve(&s),
        _ => println!("Unknown day"),
    }
}