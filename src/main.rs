use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Please pass name of file to read in.");
    } else {
        let path_graph = read_file(&args[1]);
        println!("{:?}", path_graph);
        let (subsolutions, max_weight) = wis(&path_graph);
        println!("solution: {max_weight}");
        println!("subsolutions: {:?}", subsolutions);
        // TODO: reconstruct vertices that are in the solution
    }
}

/// Computes the total weight of a max-weight independent set of
/// a path graph `pg`
fn wis(pg: &Vec<u32>) -> (Vec<u32>, u32) {
    // for readability
    let n = pg.len();
    // output array with length n + 1
    let mut a: Vec<u32> = vec![0; n + 1];
    // Base cases
    a[0] = 0; // max weight for zero vertices is zero
    a[1] = pg[0]; // max weight for one vertex subgraph is the weight of the first
    // iterate over the remainder of indices of a
    for i in 2..n + 1 {
        // take the max of:
        //   the (i - 1)th solution
        //   the (i - 2)th solution plus the ith element of the graph
        a[i] = max(a[i - 1], a[i - 2] + pg[i - 1]);
        // println!("{i}th element of path graph: {:?}", pg[i - 1]);
        // println!("subsolution for {i} vertices: {:?}", a[i]);
    }
    // final answer is the answer for n vertices
    let solution = a[n];
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
