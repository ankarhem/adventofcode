use anyhow::anyhow;
use std::str::FromStr;
use winnow::ascii::digit1;
use winnow::combinator::{alt, repeat};
use winnow::stream::Accumulate;
use winnow::token::take_while;
use winnow::{PResult, Parser};

#[derive(Debug)]
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

fn parse_diskmap(input: &mut &str) -> PResult<DiskMap> {
    let disk_map = repeat(1.., alt(
        (
            digit1.map(|d: &str| {
                DiskBlock::File { id: usize::from_str(&d[0..1]).unwrap(), size: d.len() as u32 }
            }),
            take_while(1.., |c| c == '.').map(|d: &str| DiskBlock::Free(d.len() as u32))
        )
    )).parse_next(input)?;

    Ok(DiskMap(disk_map))
}

impl FromStr for DiskMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s;
        parse_diskmap(&mut input).map_err(|e| anyhow!("{}", e))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0..111....22222")]
    #[case("00...111...2...333.44.5555.6666.777.888899")]
    fn can_parse_to_and_from_str_mini(#[case] disk_map_str: &str) {
        let parsed: DiskMap = disk_map_str.parse().unwrap();
        let actual = parsed.to_string();
        assert_eq!(disk_map_str, actual);
    }

    // #[test]
    // fn defragment_mini() {
    //     let input: DiskMap = "0..111....22222".parse().unwrap();
    //     let actual = input.defragmented().collect::<DiskMap>();
    //     assert_eq!("022111222......", actual.to_string());
    // }
}