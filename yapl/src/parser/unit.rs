// Unit functions expect they are called on correct place.

use crate::common::symbol::Symbol;

use super::stream::Stream;
use super::symbol::SymbolType;

// TODO: somehow remove repeating code.

#[derive(PartialEq)]
enum SPS {
    None,
    Slash,
    Exit,
}
pub fn string(chars: &mut Stream) -> Result<String, String> {
    let mut result = String::new();
    let next = chars.next().unwrap();
    assert!(next == '"');
    let mut state = SPS::None;
    while state != SPS::Exit {
        let Some(next) = chars.next() else {
            return Err("string end isn't found".to_string())
        };
        match state {
            SPS::None => match next {
                '\\' => state = SPS::Slash,
                '"' => state = SPS::Exit,
                _ => result.push(next),
            },
            SPS::Slash => {
                match next {
                    '\\' | '"' => result.push(next),
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    _ => return Err("unexpected symbol".to_string()),
                }
                state = SPS::None
            }
            SPS::Exit => panic!(),
        }
    }
    Ok(result)
}

pub fn chain(chars: &mut Stream) -> Result<Vec<Symbol>, String> {
    let mut result = Vec::new();
    let mut s = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Dot => {
                if s.len() == 0 {
                    return Err(err_exp_id());
                }
                result.push(Symbol::from(s));
                s = String::new();
                chars.next().unwrap();
            }
            SymbolType::Inner | SymbolType::Quote => return Err(err_exp_wh()),
            SymbolType::Letter(_) | SymbolType::Digit(_) => s.push(chars.next().unwrap()),
            SymbolType::Other => return Err(err_unsupported_symbol()),
            _ => break,
        }
    }
    if s.len() == 0 {
        return Err(err_exp_id());
    }
    result.push(Symbol::from(s));
    Ok(result)
}

pub fn special(chars: &mut Stream) -> Result<Symbol, String> {
    let mut result = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Inner | SymbolType::Quote => return Err(err_exp_wh()),
            SymbolType::Special(_) => result.push(chars.next().unwrap()),
            SymbolType::Other | SymbolType::Dot => return Err(err_unsupported_symbol()),
            _ => break,
        }
    }
    Ok(Symbol::from(result))
}

pub fn int(chars: &mut Stream) -> Result<i64, String> {
    let mut result = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Inner | SymbolType::Quote => return Err(err_exp_wh()),
            SymbolType::Digit(_) => result.push(chars.next().unwrap()),
            SymbolType::Other | SymbolType::Dot => return Err(err_unsupported_symbol()),
            _ => break,
        }
    }
    result.parse::<i64>().map_err(|e| e.to_string())
}

fn err_unsupported_symbol() -> String {
    "unsupported symbol".to_string()
}
fn err_exp_wh() -> String {
    "expected whitespace".to_string()
}
fn err_exp_id() -> String {
    "expected identifier".to_string()
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
    fn test_chain() {
        assert_eq("a2_b4.e6", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b4.e6.e8", Ok(vec!["a2_b4", "e6", "e8"]));
        assert_eq("a2_b4.e6.e8 e9", Ok(vec!["a2_b4", "e6", "e8"]));
        assert_eq("a2_b4.e6!!!e9", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b4.e6<=>e9", Ok(vec!["a2_b4", "e6"]));
        assert_eq("a2_b.4(e6)<=>e9", Ok(vec!["a2_b", "4"]));

        assert_eq!(chain(&mut Stream::new("a2_b4..e6")), Err(err_exp_id()));
        assert_eq!(chain(&mut Stream::new(".a2_b4.e6.e8")), Err(err_exp_id()));
        assert_eq!(chain(&mut Stream::new("a2_4\".e\".e8")), Err(err_exp_wh()));
    }

    #[test]
    fn test_string() {
        let sample = "\"abc \\\\ \\t \t \n \\\" \" abc";
        let result = string(&mut Stream::new(sample));
        let expected = Ok("abc \\ \t \t \n \" ".to_string());
        assert!(result == expected);
    }
}
