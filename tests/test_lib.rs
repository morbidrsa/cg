use csv::StringRecord;
use cg::{Config, run};

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
