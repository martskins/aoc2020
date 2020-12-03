use crate::{io::read_lines, result::Result};

#[derive(Debug)]
pub struct Entry {
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

pub struct Puzzle;

impl crate::PuzzleRunner for Puzzle {
    const DAY: usize = 2;
    type First = usize;
    type Second = usize;
    type Input = Vec<Entry>;

    fn parse_input(&self, filename: &str) -> Result<Self::Input> {
        let entries = read_lines(filename)?.into_iter().map(Into::into).collect();
        Ok(entries)
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        entries
            .into_iter()
            .filter_map(|c| if c.valid_part_one() { Some(c) } else { None })
            .count()
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        entries
            .into_iter()
            .filter_map(|c| if c.valid_part_two() { Some(c) } else { None })
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(2, puzzle.part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(1, puzzle.part_two(&entries));
    }
}
