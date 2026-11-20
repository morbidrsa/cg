use std::error::Error;
use std::fs::read_to_string;

use cg::{run, Config};

pub fn main() -> Result<(), Box<dyn Error>> {
    let (conf, file) = Config::from_args();
    let data = read_to_string(file)?;

    let res = run(&data, conf)?;
    println!("{:?}", res);
    Ok(())
}
