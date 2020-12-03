use crate::io::read_lines;

pub struct Puzzle;

impl crate::PuzzleRunner for Puzzle {
    const DAY: usize = 1;
    type First = i64;
    type Second = i64;
    type Input = Vec<i64>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        let lines: Vec<i64> = read_lines(filename)?
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(lines)
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        for i in entries.iter() {
            for j in entries.iter() {
                if i + j == 2020 {
                    return i * j;
                }
            }
        }

        return 0;
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        for i in entries.iter() {
            for j in entries.iter() {
                for k in entries.iter() {
                    if i + j + k == 2020 {
                        return i * j * k;
                    }
                }
            }
        }

        return 0;
    }
}
