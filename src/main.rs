use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Please pass name of file to read in.");
    } else {
        let path_graph = read_file(&args[1]);
        // println!("{:?}", path_graph);
        let (subsolutions, max_weight) = wis(&path_graph);
        println!("solution: {max_weight}");
        // println!("subsolutions: {:?}", subsolutions);
        let sln_verts = wis_reconstruction(&path_graph, &subsolutions);
        println!("Vertices involved are at indices {:?}", sln_verts);
        println!("(Using zero-indexing, so they are one off from the book's solution)");
    }
}

/// Computes the total weight of a max-weight independent set of
/// a path graph `pg`
fn wis(pg: &[u32]) -> (Vec<u32>, u32) {
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

/// Reconstructs the vertex indices from the array of subsolutions created
/// by `wis`
/// Returns an array of indices for pg that represent the vertices that are
/// part of the solution
fn wis_reconstruction(pg: &[u32], subsolutions: &[u32]) -> Vec<u32> {
    let mut s: Vec<u32> = Vec::with_capacity(subsolutions.len() / 2);
    // Start at the greatest index of subsolutions
    let mut i = subsolutions.len() - 1;
    while i >= 2 {
        if subsolutions[i - 1] >= subsolutions[i - 2] + pg[i - 1] {
            i -= 1;
        } else {
            s.push(
                (i - 1)
                    .try_into()
                    .expect("More vertices than the 32 bit integer limit"),
            );
            i -= 2;
        }
    }
    if i == 1 {
        s.push(0);
    }
    s
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
