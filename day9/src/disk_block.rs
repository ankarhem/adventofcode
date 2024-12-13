use anyhow::anyhow;
use itertools::Itertools;
use std::str::FromStr;
use winnow::stream::Accumulate;

#[derive(Debug, Clone)]
pub enum DiskBlock {
    File {
        id: usize,
        size: u32,
    },
    Free(u32),
}
impl std::fmt::Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskBlock::File { id, size } => write!(f, "{}", format!("{id}").repeat(*size as usize)),
            DiskBlock::Free(size) => write!(f, "{}", ".".repeat(*size as usize)),
        }
    }
}

#[derive(Debug)]
pub struct DiskMap(Vec<DiskBlock>);
impl std::fmt::Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.0 {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}
impl Accumulate<(DiskBlock, Option<DiskBlock>)> for DiskMap {
    fn initial(capacity: Option<usize>) -> Self {
        DiskMap(Vec::with_capacity(capacity.unwrap_or(0)))
    }

    fn accumulate(&mut self, acc: (DiskBlock, Option<DiskBlock>)) {
        self.0.push(acc.0);
        if let Some(free_space) = acc.1 {
            self.0.push(free_space);
        }
    }
}
impl FromIterator<DiskBlock> for DiskMap {
    fn from_iter<T: IntoIterator<Item=DiskBlock>>(iter: T) -> Self {
        let disk_map = iter.into_iter().collect::<Vec<DiskBlock>>();
        DiskMap(disk_map)
    }
}

fn parse_diskmap(input: &str) -> anyhow::Result<DiskMap> {
    let chunks = input.chars().chunk_by(|c| *c);

    chunks.into_iter().map(|(c, group)| {
        match c {
            '.' => Ok(DiskBlock::Free(group.count() as u32)),
            _ => {
                let id = c.to_digit(10).ok_or(anyhow!("Invalid digit"))?;
                Ok(DiskBlock::File { id: id as usize, size: group.count() as u32 })
            }
        }
    }).collect()
}

impl FromStr for DiskMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s;
        parse_diskmap(&mut input).map_err(|e| anyhow!("{}", e))
    }
}

struct DefragmentedIter<'a> {
    iter: &'a [DiskBlock],
    current_start_idx: usize,
    current_end_idx: usize,
    current_filled_free_size: u32,
    current_defragmented_size: u32,
    file_size_remaining: u32,
    free_size_remaining: u32,
}

impl DiskMap {
    fn defragmented(&self) -> DefragmentedIter {
        DefragmentedIter {
            iter: &self.0,
            current_start_idx: 0,
            current_end_idx: &self.0.len() - 1,
            current_filled_free_size: 0,
            current_defragmented_size: 0,
            file_size_remaining: self.0.iter().filter_map(|block| {
                match block {
                    DiskBlock::File { size, .. } => Some(size),
                    _ => None,
                }
            }).sum::<u32>(),
            free_size_remaining: self.0.iter().filter_map(|block| {
                match block {
                    DiskBlock::Free(size) => Some(size),
                    _ => None,
                }
            }).sum::<u32>(),
        }
    }
}

impl<'a> Iterator for DefragmentedIter<'a> {
    type Item = DiskBlock;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.file_size_remaining, &self.free_size_remaining.clone()) {
            (0, 0) => None,
            (0, free_size) => {
                self.free_size_remaining = 0;
                Some(DiskBlock::Free(*free_size))
            }
            (_, _) => {
                match &self.iter[self.current_start_idx] {
                    DiskBlock::Free(size) => {
                        // Find the next file block from end
                        let size_to_fill = *size - self.current_filled_free_size;
                        let file_block = &self.iter[self.current_end_idx];
                        while let DiskBlock::Free(_) = file_block {
                            self.current_end_idx -= 1;
                        }
                        match file_block {
                            DiskBlock::File { id, size: og_size } => {
                                let size = og_size - self.current_defragmented_size;
                                
                                if size_to_fill > size {
                                    self.file_size_remaining -= size;
                                    self.current_filled_free_size += size;
                                    self.current_defragmented_size = 0;
                                    self.current_end_idx -= 1;
                                    return Some(DiskBlock::File { id: *id, size });
                                }


                                self.current_start_idx += 1;
                                self.current_filled_free_size = 0;
                                if size_to_fill == size {
                                    self.file_size_remaining -= size;
                                    self.current_defragmented_size = 0;
                                    self.current_end_idx -= 1;
                                    return Some(DiskBlock::File { id: *id, size });
                                }

                                self.file_size_remaining -= size_to_fill;
                                self.current_defragmented_size += size_to_fill;
                                Some(DiskBlock::File { id: *id, size: size_to_fill })
                            }
                            _ => unreachable!("Expected file block"),
                        }
                    }
                    DiskBlock::File { id, size } => {
                        self.current_start_idx += 1;
                        self.file_size_remaining -= size;
                        Some(DiskBlock::File { id: *id, size: *size })
                    },
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0..111....22222")]
    #[case("00...111...2...333.44.5555.6666.777.888899")]
    fn can_parse_to_and_from_str(#[case] disk_map_str: &str) {
        let parsed: DiskMap = disk_map_str.parse().unwrap();
        let actual = parsed.to_string();
        assert_eq!(disk_map_str, actual);
    }

    #[test]
    fn defragment_mini() {
        let input: DiskMap = "0..111....22222".parse().unwrap();
        let actual = input.defragmented().collect::<DiskMap>();
        assert_eq!("022111222......", actual.to_string());
    }
}