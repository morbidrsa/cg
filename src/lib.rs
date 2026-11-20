use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::error::Error;
use std::vec::Vec;

pub struct Config {
    pub row: String,
    pub col: String,
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
        .get(conf.row.as_str())
        .ok_or("Column not found")?;

    let mut record = StringRecord::new();

    let mut results = Vec::new();
    while rdr.read_record(&mut record)? {
        if let Some(val) = record.get(target_col_idx) {
            if val == conf.col {
                results.push(record.to_owned());
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic01() {
        let data = "name,age,city\nAlice,30,Berlin\nBob,25,Munich";
        let exp = StringRecord::from(vec!["Alice", "30", "Berlin"]);
        let conf = Config {
            row: "city".into(),
            col: "Berlin".into(),
            headers: true,
        };

        let res = run(data, conf).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], exp);
    }

    #[test]
    fn basic02() {
        let data = "name,age,city\nAlice,30,Berlin\nBob,25,Berlin";
        let exp1 = StringRecord::from(vec!["Alice", "30", "Berlin"]);
        let exp2 = StringRecord::from(vec!["Bob", "25", "Berlin"]);
        let conf = Config {
            row: "city".into(),
            col: "Berlin".into(),
            headers: true,
        };

        let res = run(data, conf).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], exp1);
        assert_eq!(res[1], exp2);
    }
}
