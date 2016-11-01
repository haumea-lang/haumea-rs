//! Tests for `haumea::scanner`
extern crate haumea;
use haumea::scanner::*;
use haumea::scanner::Token::*;

fn assert_scan_is(source: &str, expected: Vec<Token>) {
    let scanner = Scanner::new(source);
    let found: Vec<Token> = scanner.collect();
    assert_eq!(found, expected);
}

#[test]
fn test_scanner_simple() {
    assert_scan_is("1+1", vec![Number(1), Operator("+".to_string()), Number(1)]);
    assert_scan_is("   1      +      
    1         ", vec![Number(1), Operator("+".to_string()), Number(1)]);
    assert_scan_is("foo * I_love_bars", vec![Ident("foo".to_string()), Operator("*".to_string()), Ident("I_love_bars".to_string())]);
}

#[test]
fn test_keywords() {
    let keywords = vec!["to", "with", "is", "return", "do", "end",
                        "if", "then", "else", "let", "be", "forever",
                        "while", "for", "each", "in",
                        "set", "to", "through", "change", "by", "variable"];
    let keywords: Vec<Token> = keywords.iter().map(|kw| Keyword(kw.to_string())).collect();
    assert_scan_is("to with is return do end if then else let be forever while for each in set to through change by variable", keywords);                              
}

#[test]
fn test_comments() {
     assert_scan_is("/* This is a comment 
     */ 1 /*So is this */ + /* And this*/ 1", vec![Number(1), Operator("+".to_string()), Number(1)]);
     assert_scan_is("/* This /* is /* a /* very */ nested */ comment */ */ 1+1", vec![Number(1), Operator("+".to_string()), Number(1)]);
}

