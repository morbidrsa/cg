use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::error::Error;
use std::vec::Vec;

pub fn main() -> Result<(), Box<dyn Error>> {
    let data = "name,age,city\nAlice,30,Berlin\nBob,25,Munich";

    let res = run(data, "city", "Berlin", true)?;
    println!("{:?}", res);
    Ok(())
}

fn run(
    data: &str,
    row: &str,
    col: &str,
    headers: bool,
) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(headers)
        .from_reader(data.as_bytes());

    let headers = rdr.headers()?.clone();
    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    let target_col_idx = *header_map.get(row).ok_or("Column not found")?;

    let mut record = StringRecord::new();

    let mut results = Vec::new();
    while rdr.read_record(&mut record)? {
        if let Some(val) = record.get(target_col_idx) {
            if val == col {
                results.push(record.to_owned());
            }
        }
    }

    Ok(results)
}
