use std::ops::AddAssign;
use std::str::FromStr;
pub struct SizeTraverser<I: Iterator, Size> {
    iter: I,
    stack: Vec<Size>,
}

impl<I: Iterator, Size: Copy + AddAssign> SizeTraverser<I, Size> {
    fn pop(&mut self) -> Option<Size> {
        let popped = self.stack.pop();
        if let Some(size) = popped {
            if let Some(parent) = self.stack.last_mut() {
                *parent += size;
            }
        }
        return popped;
    }
}

impl<I, Size> Iterator for SizeTraverser<I, Size>
where
    I: Iterator<Item = String>,
    Size: Copy + Default + AddAssign + FromStr,
{
    type Item = Size;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.iter.next() {
            let mut words = line.split_ascii_whitespace();
            match words.next() {
                Some("$") => match words.next() {
                    Some("cd") => match words.next() {
                        Some("..") => return self.pop(),
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
                None => (),
            }
        }
        return self.pop();
    }
}

pub trait TraverseSizes: Iterator<Item = String> + Sized {
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
