use std::path::PathBuf;

use yapl::{parse, Context};

#[test]
fn basics() {
    let path: PathBuf = "tests/auxiliary/test_basics.yapl".into();
    let context = Context::new_read(path.clone()).unwrap();

    let out = path.parent().unwrap().join("test_basics.out");
    let parsed = match parse(&context) {
        Ok(r) => r,
        Err(ers) => {
            dbg!(ers);
            panic!();
        }
    };
    let result = format!("{:#?}", &parsed.roots());

    // This contains more or less all patterns, so remove unit tests.
    // Since previous version: changed `Line::span`, `LitI`->`LitInt`.
    let expected = std::fs::read_to_string(&out).unwrap();

    // `Debug` output is more convenient to read.
    // let json_file = path.parent().unwrap().join("test_basics.json");
    // let json_str = serde_json::to_string(parsed.roots()).unwrap();
    // std::fs::write(json_file, json_str).unwrap();

    assert!(result == expected);
}
