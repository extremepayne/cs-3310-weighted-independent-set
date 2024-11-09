# cs-3310-weighted-independent-set

Implementation of the dynamic programming algorithim for computing a maximum-weight independent set of vertices of a path graph for UVU CS 3310. Designed as a solution to Tim Roughgarden's Algorithms Illuminated problem 16.6.

## Usage

Pass a single filename to read in that file as a path graph. First line is number of vertices, all following lines are the weights of each vertex, in order from one end of the graph to the other. See test data in [data](data/) for examples of the format. These examples are taken from [algorithmsilluminated.org](https://algorithmsilluminated.org).

## Lessons Learned

I learned a lot about dynamic programming while prepping for this assignment. Going in with all that knowledge already helped a lot with finishing this assignment in a timely manner. Plus, this time I actually gave some thought to my data structures before implementing the algorithm. Since the graphs are path graphs, they can be represented much more efficiently with a simple vector. Plus that means I can go dependency-free--no need for petgraph! And it wasn't really using anything too new in terms of Rust features, so that wasn't a cognitive burden either.

I would say the biggest Rust thing with this assignment was getting to grips with `u32` vs `usize` properly. You have to index collections with `usize`, since the collections have pointer-sized indices. Any variable you intend to index with should therefore be `usize`. However, I prefer using `Vec`s of `u32`s, because it makes me feel good inside knowing that my code will not run into unexpected behavioral differences on 32 bit systems. Luckily, casting between them is indeed easy, though you have to be explicit about it. Perhaps I should have made the reconstructed indices vector `Vec<usize>`, since its purpose is to index another vector. But for the ones that represented weights, I think I made the right choice.
