use std::error::Error;
use std::fs::read_to_string;

use cg::{run, Config};

pub fn main() -> Result<(), Box<dyn Error>> {
    let (conf, file) = Config::from_args();
    let data = read_to_string(file)?;

    let res = run(&data, conf)?;

    for row in res {
        let mut first = true;

        for col in row.iter() {
            if first {
                first = false;
                print!("{}", col);
            } else {
                print!(",{}", col);
            }
        }

        println!();
    }

    Ok(())
}
