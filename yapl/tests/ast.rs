use std::path::PathBuf;

use yapl::{parse, parser2ast, File};

#[test]
fn ast() {
    // This should be checked by `tests/parser.rs`.
    let path: PathBuf = "tests/auxiliary/test_ast.yapl".into();
    let context = File::new_read(path.clone()).unwrap();
    let parsed = match parse(&context) {
        Ok(p) => p,
        Err(e) => {
            dbg!(e);
            panic!()
        }
    };
    let ast = parser2ast(&parsed).unwrap();

    let mut context = yapl::ast::context::ContextPart::default();
    let funcs: Vec<_> = ast.into_iter().map(|l| l.act(&mut context)).collect();

    let out = path.parent().unwrap().join("test_ast.out");
    let result = format!("{:#?}", &funcs);
    // std::fs::write(&out, &result).unwrap();
    let expected = std::fs::read_to_string(&out).unwrap();
    assert!(result == expected);
}
