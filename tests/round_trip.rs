extern crate urlencoded;

use std::collections::HashMap;

#[test]
fn test_empty_query_round_trip() {
    let data = "";
    let answer = urlencoded::parse(data);
    assert!(answer.is_err());
}

#[test]
fn test_query_round_trip() {
    let data = "band=arctic%20monkeys&band=mumford%20%26%20sons&band=temper trap&color=green";
    let answer = urlencoded::parse(data).unwrap();

    let mut control = HashMap::new();
    control.insert("band".to_string(),
                   vec!["arctic monkeys".to_string(), "mumford & sons".to_string(), "temper trap".to_string()]);
    control.insert("color".to_string(), vec!["green".to_string()]);
    assert_eq!(answer, control);
}