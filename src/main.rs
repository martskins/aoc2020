mod day_one;
mod io;
mod result;

use crate::result::Result;

fn main() -> Result<()> {
    day_one::run()?;

    Ok(())
}
