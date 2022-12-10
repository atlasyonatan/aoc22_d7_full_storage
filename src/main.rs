use std::fs::File;
use std::io::{self, BufRead};
use std::ops::AddAssign;
use std::path::Path;
use std::str::FromStr;

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

struct SizeTraverser<I: Iterator, Size> {
    iter: I,
    stack: Vec<Size>,
}

impl<I, Size> Iterator for SizeTraverser<I, Size>
where
    I: Iterator<Item = String>,
    Size : Copy + Default + AddAssign + FromStr
{
    type Item = Size;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.iter.next() {
            let mut words = line.split_ascii_whitespace();
            match words.next() {
                Some("$") => match words.next() {
                    Some("cd") => match words.next() {
                        Some("..") => {
                            let popped = self.stack.pop();
                            if let Some(size) = popped {
                                if let Some(parent) = self.stack.last_mut() {
                                    *parent += size;
                                }
                                return Some(size);
                            }
                            return popped;
                        }
                        Some(_) => self.stack.push(Size::default()),
                        None => panic!("Invalid command. Missing 1 paremeter"),
                    },
                    Some("ls") => (),
                    Some(command) => panic!("Invalid command '{}'", command),
                    None => panic!("Invalid line. Missing command"),
                },
                Some("dir") => (),
                Some(s) => {
                    if let Ok(size) = s.parse::<Size>() {
                        if let Some(parent) = self.stack.last_mut() {
                            *parent += size;
                        }
                    }
                }
                None => {}
            }
        }
        return None;
    }
}

trait TraverseSizes: Iterator<Item = String> + Sized {
    fn traverse_sizes<Size>(self) -> SizeTraverser<Self, Size>;
}

impl<I> TraverseSizes for I
where
    I: Iterator<Item = String>,
{
    fn traverse_sizes<Size>(self) -> SizeTraverser<Self, Size> {
        SizeTraverser {
            iter: self,
            stack: Vec::new(),
        }
    }
}
