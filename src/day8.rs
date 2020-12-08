use crate::{io::read_lines, PuzzleRunner};

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl From<String> for Instruction {
    fn from(input: String) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        let val = parts[1].parse().unwrap();
        match parts[0] {
            "nop" => Instruction::Nop(val),
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            _ => panic!("invalid instruction"),
        }
    }
}

pub struct Puzzle;

impl PuzzleRunner for Puzzle {
    const DAY: usize = 8;
    type First = isize;
    type Second = isize;
    type Input = Vec<Instruction>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        Ok(read_lines(filename)?
            .into_iter()
            .map(Instruction::from)
            .collect())
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        let mut executed = vec![];
        let mut acc = 0;
        let mut idx = 0;
        while !executed.contains(&idx) {
            executed.push(idx);
            match entries[idx] {
                Instruction::Nop(_) => idx += 1,
                Instruction::Acc(a) => {
                    acc += a;
                    idx += 1
                }
                Instruction::Jmp(j) => idx = (idx as isize + j) as usize,
            }
        }

        acc
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        for (idx, instr) in entries.iter().enumerate() {
            match instr {
                Instruction::Acc(_) => {}
                Instruction::Jmp(i) => {
                    let mut instructions = entries.clone();
                    instructions[idx] = Instruction::Nop(i.clone());
                    match run(&instructions) {
                        Some(acc) => return acc,
                        None => {}
                    }
                }
                Instruction::Nop(i) => {
                    let mut instructions = entries.clone();
                    instructions[idx] = Instruction::Jmp(i.clone());
                    match run(&instructions) {
                        Some(acc) => return acc,
                        None => {}
                    }
                }
            }
        }

        unreachable!("should have a solution");
    }
}

fn run(instructions: &[Instruction]) -> Option<isize> {
    let mut executed = vec![];
    let mut acc = 0;
    let mut idx = 0;
    while !executed.contains(&idx) {
        executed.push(idx);
        if idx == instructions.len() {
            return Some(acc);
        }

        match instructions[idx] {
            Instruction::Nop(_) => idx += 1,
            Instruction::Acc(a) => {
                acc += a;
                idx += 1
            }
            Instruction::Jmp(j) => idx = (idx as isize + j) as usize,
        }
    }

    return None;
}
#[cfg(test)]
mod test {
    use super::Puzzle;
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(5, puzzle.part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(8, puzzle.part_two(&entries));
    }
}
