use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Please pass name of file to read in.");
    } else {
        let path_graph = read_file(&args[1]);
        println!("{:?}", path_graph);
        // TODO: process data, print result
    }
}

/// Computes the total weight of a max-weight independent set of
/// a path graph `pg`
fn wis(pg: &Vec<u32>) -> (Vec<u32>, u32) {
    let mut a: Vec<u32> = Vec::with_capacity(pg.len());
    // TODO: actually compute the solutions
    // Base cases
    a[0] = 0;
    a[1] = pg[0];
    let solution = a[pg.len() - 1];
    (a, solution)
}

/// Reads in a test case file that describes a path graph and constructs a list
fn read_file(filename: &str) -> Vec<u32> {
    println!("reading data file: {filename}");
    let data = File::open(filename).unwrap();
    let mut data_reader = io::BufReader::new(data).lines();
    let data_size: usize = data_reader
        .next()
        .expect("No vector size provided")
        .unwrap()
        .parse()
        .expect("First line is not a valid path graph size");
    let mut data_vec: Vec<u32> = Vec::with_capacity(data_size);
    for weight in data_reader.flatten() {
        data_vec.push(
            weight
                .parse()
                .expect("Provided weight is either not a positive integer or is too large"),
        );
    }
    data_vec
}
