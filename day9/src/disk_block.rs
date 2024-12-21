use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Mul;
use std::str::FromStr;
use winnow::stream::Accumulate;

#[derive(Debug, Clone)]
pub enum DiskBlock {
    File { id: usize, size: u32 },
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
    fn from_iter<T: IntoIterator<Item = DiskBlock>>(iter: T) -> Self {
        let disk_map = iter.into_iter().collect::<Vec<DiskBlock>>();
        DiskMap(disk_map)
    }
}

fn parse_diskmap(input: &str) -> anyhow::Result<DiskMap> {
    let chunks = input.chars().chunk_by(|c| *c);

    chunks
        .into_iter()
        .map(|(c, group)| match c {
            '.' => Ok(DiskBlock::Free(group.count() as u32)),
            _ => {
                let id = c.to_digit(10).ok_or(anyhow!("Invalid digit"))?;
                Ok(DiskBlock::File {
                    id: id as usize,
                    size: group.count() as u32,
                })
            }
        })
        .collect()
}

impl FromStr for DiskMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s;
        parse_diskmap(input).map_err(|e| anyhow!("{}", e))
    }
}

pub struct FragmentedIter<'a> {
    iter: &'a [DiskBlock],
    current_start_idx: usize,
    current_end_idx: usize,
    current_filled_free_size: u32,
    current_fragmented_file: Option<(usize, u32)>,
    file_size_remaining: u32,
    free_size_remaining: u32,
}

impl DiskMap {
    pub fn fragmented(&self) -> FragmentedIter {
        FragmentedIter {
            iter: &self.0,
            current_start_idx: 0,
            current_end_idx: &self.0.len() - 1,
            current_filled_free_size: 0,
            current_fragmented_file: None,
            file_size_remaining: self
                .0
                .iter()
                .filter_map(|block| match block {
                    DiskBlock::File { size, .. } => Some(size),
                    _ => None,
                })
                .sum::<u32>(),
            free_size_remaining: self
                .0
                .iter()
                .filter_map(|block| match block {
                    DiskBlock::Free(size) => Some(size),
                    _ => None,
                })
                .sum::<u32>(),
        }
    }

    pub fn checksum(&self) -> u128 {
        self.0
            .iter()
            .flat_map(|block| match block {
                DiskBlock::Free(size) => (0..*size)
                    .map(|_| DiskBlock::Free(1))
                    .collect::<Vec<DiskBlock>>(),
                DiskBlock::File { id, size } => (0..*size)
                    .map(|_| DiskBlock::File { id: *id, size: 1 })
                    .collect::<Vec<DiskBlock>>(),
            })
            .enumerate()
            .map(|(i, block)| match block {
                DiskBlock::Free(_) => 0,
                DiskBlock::File { id, size } => id.mul(i) as u128,
            })
            .sum()
    }
}

impl<'a> Iterator for FragmentedIter<'a> {
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
                        let mut file_block = &self.iter[self.current_end_idx];
                        while let DiskBlock::Free(_) = file_block {
                            self.current_end_idx -= 1;
                            file_block = &self.iter[self.current_end_idx];
                        }
                        match file_block {
                            DiskBlock::File { id, size: og_size } => {
                                let size =
                                    og_size - self.current_fragmented_file.unwrap_or((0, 0)).1;

                                if size_to_fill > size {
                                    self.file_size_remaining -= size;
                                    self.current_filled_free_size += size;
                                    self.current_fragmented_file = None;
                                    self.current_end_idx -= 1;
                                    return Some(DiskBlock::File { id: *id, size });
                                }

                                self.current_start_idx += 1;
                                self.current_filled_free_size = 0;
                                if size_to_fill == size {
                                    self.file_size_remaining -= size;
                                    self.current_fragmented_file = None;
                                    self.current_end_idx -= 1;
                                    return Some(DiskBlock::File { id: *id, size });
                                }

                                self.file_size_remaining -= size_to_fill;
                                self.current_fragmented_file = Some((
                                    *id,
                                    self.current_fragmented_file.unwrap_or((0, 0)).1 + size_to_fill,
                                ));
                                Some(DiskBlock::File {
                                    id: *id,
                                    size: size_to_fill,
                                })
                            }
                            _ => unreachable!("Expected file block"),
                        }
                    }
                    DiskBlock::File { id, size } => {
                        self.current_start_idx += 1;
                        let size = match self.current_fragmented_file {
                            Some((fragmented_id, fragmented_size)) => {
                                if id == &fragmented_id {
                                    let size = size - fragmented_size;
                                    self.current_fragmented_file = None;
                                    size
                                } else {
                                    *size
                                }
                            }
                            None => *size,
                        };
                        self.file_size_remaining -= size;
                        Some(DiskBlock::File { id: *id, size })
                    }
                }
            }
        }
    }
}

pub struct DefragmentedIter<'a> {
    iter: &'a [DiskBlock],
    current_idx: usize,
    used_file_ids: HashSet<usize>,
    current_filled_free_size: u32,
}

impl DiskMap {
    pub fn defragmented(&self) -> DefragmentedIter {
        DefragmentedIter {
            iter: &self.0,
            current_idx: 0,
            current_filled_free_size: 0,
            used_file_ids: HashSet::new(),
        }
    }
}

impl Iterator for DefragmentedIter<'_> {
    type Item = DiskBlock;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_idx < self.iter.len() {
            let block = &self.iter[self.current_idx];

            return match block {
                DiskBlock::File { id, size } => {
                    self.current_idx += 1;
                    if self.used_file_ids.contains(id) {
                        return Some(DiskBlock::Free(*size));
                    }
                    self.used_file_ids.insert(*id);
                    Some(DiskBlock::File {
                        id: *id,
                        size: *size,
                    })
                }
                DiskBlock::Free(size) => {
                    let size = *size - self.current_filled_free_size;
                    let defragmented_block = self.iter.iter().rev().find(|block| match block {
                        DiskBlock::File {
                            id: candidate_id,
                            size: candidate_size,
                        } => candidate_size <= &size && !self.used_file_ids.contains(candidate_id),
                        _ => false,
                    });

                    match defragmented_block {
                        Some(DiskBlock::File {
                            id: defragmented_id,
                            size: defragmented_size,
                        }) => {
                            if defragmented_size == &size {
                                self.current_idx += 1;
                                self.current_filled_free_size = 0;
                            } else {
                                self.current_filled_free_size += defragmented_size;
                            }
                            self.used_file_ids.insert(*defragmented_id);
                            Some(defragmented_block.unwrap().clone())
                        }
                        _ => {
                            self.current_idx += 1;
                            self.current_filled_free_size = 0;
                            Some(DiskBlock::Free(size))
                        }
                    }
                }
            };
        }
        None
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
    fn fragment_mini() {
        let input: DiskMap = "0..111....22222".parse().unwrap();
        let actual = input.fragmented().collect::<DiskMap>();
        assert_eq!("022111222......", actual.to_string());
    }

    #[test]
    fn fragment_example() {
        let input: DiskMap = "00...111...2...333.44.5555.6666.777.888899"
            .parse()
            .unwrap();
        let actual = input.fragmented().collect::<DiskMap>();
        assert_eq!(
            "0099811188827773336446555566..............",
            actual.to_string()
        );
    }

    #[test]
    fn defragment_example() {
        let input: DiskMap = "00...111...2...333.44.5555.6666.777.888899"
            .parse()
            .unwrap();
        let actual = input.defragmented().collect::<DiskMap>();
        assert_eq!(
            "00992111777.44.333....5555.6666.....8888..",
            actual.to_string()
        );
    }

    #[test]
    fn checksum_fragmented() {
        let input: DiskMap = "0099811188827773336446555566.............."
            .parse()
            .unwrap();
        let actual = input.checksum();
        assert_eq!(1928, actual);
    }

    #[test]
    fn checksum_defragmented() {
        let input: DiskMap = "00992111777.44.333....5555.6666.....8888.."
            .parse()
            .unwrap();
        let actual = input.checksum();
        assert_eq!(2858, actual);
    }
}
