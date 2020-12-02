use std::{
    fs::File,
    io::{BufRead, Result},
};

pub fn read_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    reader.lines().collect()
}
