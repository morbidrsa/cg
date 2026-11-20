use cg::{run, Config};
use csv::StringRecord;
use std::error::Error;
use std::fs::read_to_string;

#[test]
fn basic01() {
    let data = "name,age,city\nAlice,30,Berlin\nBob,25,Munich";
    let exp = StringRecord::from(vec!["Alice", "30", "Berlin"]);
    let conf = Config {
        col: "city".into(),
        val: "Berlin".into(),
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
        col: "city".into(),
        val: "Berlin".into(),
        headers: true,
    };

    let res = run(data, conf).unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], exp1);
    assert_eq!(res[1], exp2);
}

#[test]
fn basic03() -> Result<(), Box<dyn Error>> {
    let data = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test01.csv"))?;
    let exp = StringRecord::from(vec!["Alice", "30", "Berlin"]);
    let conf = Config {
        col: "city".into(),
        val: "Berlin".into(),
        headers: true,
    };

    let res = run(&data, conf)?;
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], exp);

    Ok(())
}
