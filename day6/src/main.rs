use anyhow::Result;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    width: isize,
    height: isize,
    obstacles: Vec<(isize, isize)>,
    guard_start_position: (isize, isize),
    guard_position: (isize, isize),
    guard_direction: Direction,
    guard_has_moved: bool,
}

impl Map {
    fn turn_right(&mut self) {
        self.guard_direction = match self.guard_direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn next_position(&self) -> Result<Option<(isize, isize)>> {
        let (x, y) = self.guard_position;
        let next = match self.guard_direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if next.0 < 0 || next.0 >= self.width || next.1 < 0 || next.1 >= self.height {
            return Err(anyhow::anyhow!("Next position is out of bounds"));
        }

        if self.obstacles.contains(&next) {
            Ok(None)
        } else {
            Ok(Some(next))
        }
    }

    fn goto(&mut self, (x, y): (isize, isize)) {
        self.guard_position = (x, y);
    }
}

impl Iterator for Map {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        // Check for loop
        if self.guard_has_moved && self.guard_position == self.guard_start_position {
            return None;
        }

        // Try all directions if needed
        for _ in 0..4 {
            match self.next_position() {
                // If we can move to the next position, do so
                Ok(Some(next)) => {
                    self.goto(next);
                    return Some(self.guard_position);
                }
                // If the next position is an obstacle, turn right
                Ok(None) => {
                    self.turn_right();
                }
                // If the next position is out of bounds, we're done
                Err(_) => {
                    return None;
                }
            }
        }

        // If we've tried all directions and found no valid move, we're done
        None
    }
}

fn parse_input(input: &str) -> Map {
    let mut guard_position = (0, 0);
    let mut guard_direction = Direction::Up;
    let mut width = 0;
    let mut height = 0;
    let mut obstacles = vec![];

    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        width = line.len() as isize;
        height = y + 1;

        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            if c == '^' {
                guard_position = (x, y);
            }
            if c == '#' {
                obstacles.push((x, y));
            }
            match c {
                '^' | 'v' | '<' | '>' => {
                    guard_position = (x, y);
                    guard_direction = match c {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => unreachable!(),
                    };
                }
                '#' => {
                    obstacles.push((x, y));
                }
                _ => (),
            }
        }
    }

    Map {
        width,
        height,
        obstacles,
        guard_start_position: guard_position,
        guard_position,
        guard_direction,
        guard_has_moved: false,
    }
}

fn part_one(input: &str) -> u32 {
    let input = parse_input(input);

    let visited: HashSet<_> = input.collect();

    visited.len() as u32
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 6, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 6, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(41, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(31, actual);
    }
}