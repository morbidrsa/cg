use std::error::Error;

use cg::{Config, run};

pub fn main() -> Result<(), Box<dyn Error>> {
    let data = "name,age,city\nAlice,30,Berlin\nBob,25,Munich";
    let conf = Config {
        row: "city".into(),
        col: "Berlin".into(),
        headers: true,
    };

    let res = run(data, conf)?;
    println!("{:?}", res);
    Ok(())
}

