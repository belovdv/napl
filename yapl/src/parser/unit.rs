// Unit functions expect they are called on correct place.

use crate::common::symbol::Symbol;

use super::stream::Stream;
use super::symbol::SymbolType;

pub fn string(chars: &mut Stream) -> Result<String, String> {
    todo!()
}

pub fn chain(chars: &mut Stream) -> Result<Vec<Symbol>, String> {
    let mut result = Vec::new();
    let mut s = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Bracket(_, _)
            | SymbolType::Comma
            | SymbolType::Special(_)
            | SymbolType::EOS
            | SymbolType::Whitespace(_) => break,
            SymbolType::Dot => {
                if s.len() == 0 {
                    return Err("expected identifier".to_string());
                }
                result.push(Symbol::from(s));
                s = String::new();
                chars.next().unwrap();
            }
            SymbolType::Inner | SymbolType::Quote => {
                return Err("expected whitespace before quote".to_string())
            }
            SymbolType::Letter(_) | SymbolType::Digit(_) => s.push(chars.next().unwrap()),
            SymbolType::NewLine => panic!(),
            SymbolType::Other => return Err("unsupported symbol".to_string()),
        }
    }
    if s.len() == 0 {
        return Err("expected identifier".to_string());
    }
    result.push(Symbol::from(s));
    Ok(result)
}

pub fn special(chars: &mut Stream) -> Result<Symbol, String> {
    chars.next();
    Ok(Symbol::from("placeholder"))
}

pub fn int(chars: &mut Stream) -> Result<i64, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq(sample: &str, expected: Result<Vec<&'static str>, String>) {
        let expected = expected.map(|v| v.into_iter().map(Symbol::from).collect());
        let result = chain(&mut Stream::new(sample));
        if result != expected {
            dbg!(sample, &result, &expected);
            panic!("test failed")
        }
    }

    #[test]
    fn chain_simple() {
        // assert_eq("", Ok(vec![])); // Incorrect use.
        assert_eq("sample", Ok(vec!["sample"]));
        assert_eq("sample1", Ok(vec!["sample1"]));
        assert_eq("sample_s", Ok(vec!["sample_s"]));
        assert_eq("sample_2s", Ok(vec!["sample_2s"]));
    }

    #[test]
    fn chain_chain() {
        assert_eq("a2_b4.e6", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b4.e6.e8", Ok(vec!["a2_b4", "e6", "e8"]));
        assert_eq("a2_b4.e6.e8 e9", Ok(vec!["a2_b4", "e6", "e8"]));
        assert_eq("a2_b4.e6!!!e9", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b4.e6<=>e9", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b.4(e6)<=>e9", Ok(vec!["a2_b", "4"]));
    }

    #[test]
    fn chain_error() {
        assert_eq!(
            chain(&mut Stream::new("a2_b4..e6")),
            Err("expected identifier".to_string())
        );
        assert_eq!(
            chain(&mut Stream::new(".a2_b4.e6.e8")),
            Err("expected identifier".to_string())
        );
        assert_eq!(
            chain(&mut Stream::new("a2_b4\".e6\".e8")),
            Err("expected whitespace before quote".to_string())
        );
    }
}
