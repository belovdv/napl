use std::path::PathBuf;

use yapl::{parse, Context};

// Disable for this commit - it will be redone in the next.
// #[test]
fn basics() {
    /*
    let path: PathBuf = "tests/auxiliary/test_basics.yapl".into();
    let context = Context::new_read(path.clone()).unwrap();

    dbg!(&context.file);
    dbg!(&context.lines);

    let parsed = parse(&context);
    let result = format!("{:#?}", &parsed);

    let out = path.parent().unwrap().join("test_basics.out");
    // To be done: fix spans.
    // Except spans (and whole awkwardness of code) - that's it.
    // This contains more or less all patterns, so remove unit tests.
    let expected = std::fs::read_to_string(out).unwrap();

    assert!(result == expected);
    */
}
