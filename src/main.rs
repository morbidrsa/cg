use csv::Reader;
use std::collections::HashMap;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let data = "name,age,city\nAlice,30,Berlin\nBob,25,Munich";

    run(data, "city", "Berlin")
}

fn run(data: &str, row: &str, col: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_reader(data.as_bytes());

    let headers = rdr.headers()?.clone();
    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    let target_col_idx = *header_map.get(row).ok_or("Column not found")?;

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        if let Some(val) = record.get(target_col_idx) {
            if val == col {
                println!("Match: {:?}", record);
            }
        }
    }

    Ok(())
}
