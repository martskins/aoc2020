use crate::io::read_lines;
use crate::result::Result;

pub fn run() -> Result<()> {
    let lines: Vec<i64> = read_lines("./data/day1.txt")?
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect();

    println!("day one answers");
    'part_one: for i in lines.iter() {
        for j in lines.iter() {
            if i + j == 2020 {
                println!("    part one: {}", i * j);
                break 'part_one;
            }
        }
    }

    'part_two: for i in lines.iter() {
        for j in lines.iter() {
            for k in lines.iter() {
                if i + j + k == 2020 {
                    println!("    part two: {}", i * j * k);
                    break 'part_two;
                }
            }
        }
    }

    Ok(())
}
