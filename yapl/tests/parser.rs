use std::path::PathBuf;

use yapl::{parse, File};

#[test]
fn parser() {
    let path: PathBuf = "tests/auxiliary/test_parser.yapl".into();
    let context = File::new_read(path.clone()).unwrap();

    let out = path.parent().unwrap().join("test_parser.out");
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

    // let out = path.parent().unwrap().join("_test_parser.out");
    // std::fs::write(out, &result).unwrap();

    assert!(result == expected);

    // let out_ast = path.parent().unwrap().join("test_basics_ast.out");
    // let ast = parser2ast(&parsed);
    // std::fs::write(out_ast, format!("{:#?}", ast)).unwrap();
}
