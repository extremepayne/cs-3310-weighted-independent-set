use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Please pass name of file to read in.");
    } else {
        // TODO: read in file, construct data, process data, print result
    }
}
