use std::fmt::Display;
use winnow::Parser;

#[derive(Debug, Clone)]
enum DiskBlock {
    File {
        id: u32,
        size: usize,
    },
    Free(usize),
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DiskBlock::File { id, size } => format!("{}", id).repeat(*size),
            DiskBlock::Free(size) => ".".repeat(*size),
        };
        write!(f, "{}", str)
    }
}

fn parse_input(input: &str) -> Vec<DiskBlock> {
    input.chars().enumerate().map(|(i, c)| {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            DiskBlock::File {
                id: (i / 2) as u32,
                size,
            }
        } else {
            DiskBlock::Free(size)
        }
    }).collect()
}

struct DefragmentedIter<'a, T> {
    blocks: &'a [T],
    current: usize,
}

trait Defragmentable<T> {
    fn defragment(&self) -> DefragmentedIter<T>;
}

impl Defragmentable<DiskBlock> for Vec<DiskBlock> {
    fn defragment(&self) -> DefragmentedIter<DiskBlock> {
        DefragmentedIter {
            blocks: self.as_slice(),
            current: 0,
        }
    }
}

impl<'a> Iterator for DefragmentedIter<'a, DiskBlock> {
    type Item = &'a DiskBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.current;
        
        match &self.blocks[i] {
            &DiskBlock::File { .. } => Some(&self.blocks[i]),
            &DiskBlock::Free(size) => {
                if size > 0 {
                    self.current += 1;
                    Some(&self.blocks[i])
                } else {
                    None
                } 
            }
        }
    }
}


fn part_one(input: &str) -> u32 {
    let input = parse_input(input);

    input
        .defragment()
        .filter(|block| match block {
            DiskBlock::File { .. } => true,
            _ => false,
        })
        .enumerate()
        .map(|(i, block)| {
            match block {
                DiskBlock::File { id , size} => {
                    *id * i as u32
                }
                DiskBlock::Free(_) => 0
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 1, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 1, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser_mini() {
        let example = "12345";
        let actual = parse_input(example).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        assert_eq!("0..111....22222", actual);
    }

    #[test]
    fn parser_example() {
        let example = include_str!("example");
        let actual = parse_input(example).iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", actual);
    }


    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(1928, actual);
    }

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}