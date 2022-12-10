use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod traverse_sizes;
use traverse_sizes::TraverseSizes;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let mut sizes: Vec<u32> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .traverse_sizes()
        .collect();

    //part 1
    let sum = sizes.iter().filter(|size| **size <= 100000).sum::<u32>();
    println!("part 1: {}", sum);

    //part 2
    let total_storage = 70_000_000;
    let need_free = 30_000_000;

    sizes.sort();
    let used = sizes.last().unwrap();
    let unused = total_storage - used;

    let min_delete_size = need_free - unused;
    let size_for_delete = sizes.iter().find(|s| **s >= min_delete_size).unwrap();
    println!("part 2: {}", size_for_delete)
}

