use std::str::Chars;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source_str: &'a str,
    source_chars: Chars<'a>,
    operator_chars: Vec<char>,
    operators: Vec<&'static str>,
    ident_chars: Vec<char>,
    reserved_words: Vec<&'static str>,
    peek: Option<char>,
}

#[derive(Debug)]
pub enum Token<'a> {
    Number(i64),
    Ident(&'a str),
    Keyword(Keyword),
    Operator(Op),
    EOF,
}

#[derive(Debug)]
pub enum Keyword {
    To,
    With,
    Is,
    A,
    An,
    Returns,
    Return,
    Do,
    End,
    If,
    Then,
    Else,
    Let,
    Be,
    Set,
    Change,
    By,
    Integer,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Equ,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
    Not,
    Lp,
    Rp,
    Inv,
    Bor,
    Band,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        let mut chars = source.chars();
        let peek = Some(" ");
        Scanner {
            source_str: source,
            source_chars: chars,
            operator_chars: vec!['+', '=', '-', '*', '/', '<', '>', '~', '|', '&', '(', ')'],
            operators: vec!["+", "=", "-", "*", "/", "<", ">", ">=", "<=",
                            "~", "|", "&", "and", "or", "not", "(", ")", "!="],
            ident_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".chars().collect::<Vec<_>>(),
            reserved_words: vec!["to", "with", "is", "a", "an", "returns", "return",
                                 "do", "end", "if", "then", "else", "let", "be",
                                 "set", "to", "change", "by", "Integer"],
            peek: peek,
        }
    }

    fn get_char(&mut self) {
        self.peek = self.chars.next();
    }

    fn skip_white(&mut self) {
        loop {
            match self.peek {
                Some(c) if c.is_whitespace() {

                }
                _ => break,
            }
        }
    }
    pub fn next(&mut self) -> Token {
        self.skip_white();
        match self.peek {
            Some(c) => {
                Token::Ident(c)
            },
            None => Token::EOF,
        }
    }
}
