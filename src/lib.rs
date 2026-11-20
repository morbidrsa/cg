use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::error::Error;
use std::vec::Vec;

pub struct Config {
    pub col: String,
    pub val: String,
    pub headers: bool,
}

pub fn run(data: &str, conf: Config) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(conf.headers)
        .from_reader(data.as_bytes());

    let headers = rdr.headers()?.clone();
    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    let target_col_idx = *header_map
        .get(conf.col.as_str())
        .ok_or("Column not found")?;

    let mut record = StringRecord::new();

    let mut results = Vec::new();
    while rdr.read_record(&mut record)? {
        if let Some(val) = record.get(target_col_idx) {
            if val == conf.val {
                results.push(record.to_owned());
            }
        }
    }

    Ok(results)
}
