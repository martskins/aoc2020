use std::collections::HashSet;

use crate::{io::read_lines, PuzzleRunner};

#[derive(Default)]
pub struct Group {
    answers: Vec<Vec<char>>,
}

pub struct Puzzle;

impl PuzzleRunner for Puzzle {
    const DAY: usize = 6;
    type First = usize;
    type Second = usize;
    type Input = Vec<Group>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        let lines = read_lines(filename)?;
        let mut groups = vec![];
        let mut group = Group::default();
        for line in lines {
            if line.is_empty() {
                groups.push(group);
                group = Group::default();
                continue;
            }

            group.answers.push(line.chars().collect());
        }
        groups.push(group);
        Ok(groups)
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        let out: Vec<HashSet<&char>> = entries
            .into_iter()
            .map(|g| g.answers.iter().flatten().collect())
            .collect();
        out.into_iter().fold(0, |acc, n| acc + n.len())
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        let out: Vec<HashSet<&char>> = entries
            .into_iter()
            .map(|g| {
                g.answers
                    .iter()
                    .enumerate()
                    .fold(HashSet::new(), |acc, (idx, n)| {
                        if idx == 0 {
                            return n.into_iter().collect();
                        }

                        HashSet::from(n.into_iter().collect())
                            .intersection(&acc)
                            .cloned()
                            .collect()
                    })
            })
            .collect();
        out.into_iter().fold(0, |acc, n| acc + n.len())
    }
}

#[cfg(test)]
mod test {
    use super::Puzzle;
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(11, puzzle.part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(6, puzzle.part_two(&entries));
    }
}
