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
pub enum Token {
    Number(i64),
    Ident(String),
    Keyword(String),
    Operator(String),
    Lp,
    Rp,
    Error(char),
    EOF,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        let chars = source.chars();
        let peek = Some(' ');
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

    pub fn next(&mut self) -> Token {
        self.skip_white();
        match self.peek {
            Some(c) => {
                if self.ident_chars.contains(&c) {
                    self.get_ident_token()
                } else if c.is_digit(10) {
                    Token::Number(self.get_num())
                } else if c == '(' {
                    self.get_char();
                    Token::Lp
                } else if c == ')' {
                    self.get_char();
                    Token::Rp
                } else if self.operator_chars.contains(&c) {
                    Token::Operator(self.get_op())
                } else {
                    Token::Error(c)
                }
            },
            None => Token::EOF,
        }
    }

    fn get_char(&mut self) {
        self.peek = self.source_chars.next();
    }

    fn skip_white(&mut self) {
        loop {
            match self.peek {
                Some(c) if c.is_whitespace() => {
                    self.get_char()
                }
                _ => break,
            }
        }
    }

    fn get_num(&mut self) -> i64 {
        let mut s = String::new();
        s.push(self.peek.unwrap());
        loop {
            self.get_char();
            match self.peek {
                Some(c) if c.is_digit(10) => s.push(c),
                _ => break,
            }
        }
        s.parse::<i64>().unwrap()
    }

    fn get_ident_token(&mut self) -> Token {
        let mut s = String::new();
        s.push(self.peek.unwrap());
        loop {
            self.get_char();
            match self.peek {
                Some(c) if self.ident_chars.contains(&c) => s.push(c),
                _ => break,
            }
        };
        if self.reserved_words.contains(&&s[..]) {
            Token::Keyword(s)
        } else if self.operators.contains(&&s[..]) {
            Token::Operator(s)
        } else {
            Token::Ident(s)
        }
    }

    fn get_op(&mut self) -> String {
        let mut s = String::new();
        s.push(self.peek.unwrap());
        loop {
            self.get_char();
            match self.peek {
                Some(c) if self.operator_chars.contains(&c) => s.push(c),
                _ => break,
            }
        };
        s
    }
}
