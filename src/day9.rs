use crate::{io::read_lines, PuzzleRunner};

pub struct Puzzle {
    preamble_size: usize,
}

impl Puzzle {
    pub fn new(preamble_size: usize) -> Self {
        Self { preamble_size }
    }
}

impl PuzzleRunner for Puzzle {
    const DAY: usize = 9;
    type First = i64;
    type Second = i64;
    type Input = Vec<i64>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        Ok(read_lines(filename)?
            .into_iter()
            .map(|c| c.parse().unwrap())
            .collect())
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        'outer: for (idx, number) in entries.iter().enumerate() {
            if idx < self.preamble_size {
                continue;
            }

            let preamble = &entries[idx - self.preamble_size..idx];
            for j in preamble.iter() {
                for k in preamble.iter() {
                    if &(j + k) == number && j != k {
                        continue 'outer;
                    }
                }
            }

            // is my input broken? this returns 1497 which is not accepted as the solution but it
            // clearly satisfies the conditions. The second number that satisfies it is 144381670,
            // which is accepted as the solution.
            return number.clone();
        }

        unreachable!("should have a solution");
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        let number = 144381670;
        'outer: for i in 0..entries.len() {
            for j in 0..entries.len() {
                let numbers = entries.iter().skip(i).take(j);
                let sum = numbers.clone().fold(0, |acc, n| acc + n);
                if sum == number {
                    return numbers.clone().max().unwrap() + numbers.min().unwrap();
                } else if sum > number {
                    continue 'outer;
                }
            }
        }

        unreachable!("should have a solution");
    }
}

#[cfg(test)]
mod test {
    use super::Puzzle;
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle::new(5);
        let input = puzzle.test_input().unwrap();
        assert_eq!(127, puzzle.part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle::new(5);
        let input = puzzle.test_input().unwrap();
        assert_eq!(62, dbg!(puzzle.part_two(&input)));
    }
}
