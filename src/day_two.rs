use crate::{io::read_lines, result::Result};

#[derive(Debug)]
struct Entry {
    num_left: usize,
    num_right: usize,
    ch: char,
    password: String,
}

impl Entry {
    fn valid_part_one(&self) -> bool {
        let occurrences = self.password.matches(self.ch).count();
        occurrences >= self.num_left && occurrences <= self.num_right
    }

    fn valid_part_two(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let first = chars.get(self.num_left - 1);
        let second = chars.get(self.num_right - 1);
        (first == Some(&self.ch) || second == Some(&self.ch)) && first != second
    }
}

impl From<String> for Entry {
    fn from(input: String) -> Self {
        let mut split = input.split_whitespace();
        let mut minmax = split.next().unwrap().split('-');
        let num_left = minmax.next().unwrap().parse().unwrap();
        let num_right = minmax.next().unwrap().parse().unwrap();
        let ch = split.next().unwrap().chars().next().unwrap();
        let password = split.next().unwrap().to_string();

        Entry {
            num_left,
            num_right,
            ch,
            password,
        }
    }
}

fn part_one(entries: &[Entry]) -> usize {
    entries
        .into_iter()
        .filter_map(|c| if c.valid_part_one() { Some(c) } else { None })
        .count()
}

fn part_two(entries: &[Entry]) -> usize {
    entries
        .into_iter()
        .filter_map(|c| if c.valid_part_two() { Some(c) } else { None })
        .count()
}

fn parse_entries(filename: &str) -> Result<Vec<Entry>> {
    let entries = read_lines(filename)?.into_iter().map(Into::into).collect();
    Ok(entries)
}

pub fn run() -> Result<()> {
    let entries = parse_entries("./data/day2.txt")?;
    println!("day two answers");
    println!("    part one: {}", part_one(&entries));
    println!("    part two: {}", part_two(&entries));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let entries = parse_entries("./data/day2_test.txt").unwrap();
        assert_eq!(2, part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let entries = parse_entries("./data/day2_test.txt").unwrap();
        assert_eq!(1, part_two(&entries));
    }
}
