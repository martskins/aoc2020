mod day_one;
mod day_two;
mod io;
mod result;

use crate::result::Result;

fn main() -> Result<()> {
    day_one::run()?;
    day_two::run()?;

    Ok(())
}
