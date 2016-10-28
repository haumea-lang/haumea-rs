/// src/scanner.rs
/// The scanner for the haumea language

use std::str::Chars; // We need to bring the Chars struct into scope

/// The scanner struct
#[derive(Debug)]
pub struct Scanner<'a> {
    /// The source &str used to create the scanner.
    ///
    /// Scanner doesn't do anything with it currently, but it is kept in case clients
    /// want to get back the source code and, more importantly,
    /// to keep it in scope so that the source_chars iterator can work
    pub source_str: &'a str,
    /// An iterator of chars over the source str
    source_chars: Chars<'a>,
    /// A vector of chars that can be in operators
    operator_chars: Vec<char>,
    /// A vector of allowed operators
    operators: Vec<&'static str>,
    /// A vector of chars that can be in identifiers
    ident_chars: Vec<char>,
    // A vector of keywords in haumea
    reserved_words: Vec<&'static str>,
    /// The look ahead char
    pub peek: Option<char>,
}

/// An enum representing the various tokens that can occur
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    /// An integer number
    ///
    /// The content is the number read as an i64
    Number(i32),
    /// An identifier
    ///
    /// The content is the name of the identifier
    Ident(String),
    /// A reserved word (or keyword)
    ///
    /// The content is the name of the keyword
    Keyword(String),
    /// An operator
    ///
    /// The content is the name of the operator
    Operator(String),
    /// Left parens
    Lp,
    /// Right parens
    Rp,
    /// A comma
    Comma,
    /// An unexpected char was read
    ///
    /// The content is the char read
    Error(char),
    /// End of input
    EOF,
}

impl<'a> Scanner<'a> {
    /// Constructs a new Scanner from a source &str
    ///
    /// # Examples
    /// ```
    /// # use haumea::scanner::{Scanner, Token};
    /// let source = "1 + 1";
    /// let scanner = Scanner::new(source);
    /// assert_eq!(scanner.source_str, source);
    /// assert_eq!(scanner.peek, Some(' '));
    /// ```
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
            reserved_words: vec!["to", "with", "is", "return", "do", "end",
                                 "if", "then", "else", "let", "be", "forever",
								 "while", "for", "each", "in",
                                 "set", "to", "change", "by", "variable"],
            peek: peek,
        }
    }

    /// Returns the next token in the source. Token::EOF means that all the input has been read
    ///
    /// # Examples
    /// ```
    /// # use haumea::scanner::{Scanner, Token};
    /// let mut s = Scanner::new("1 + 1");
    /// assert_eq!(s.next_token(), Token::Number(1));
    /// assert_eq!(s.next_token(), Token::Operator("+".to_string()));
    /// assert_eq!(s.next_token(), Token::Number(1));
    /// assert_eq!(s.next_token(), Token::EOF);
    /// ```
    pub fn next_token(&mut self) -> Token {
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
                } else if c == ',' {
                    self.get_char();
                    Token::Comma
                } else if self.operator_chars.contains(&c) {
                    Token::Operator(self.get_op())
                } else {
                    self.get_char();
                    Token::Error(c)
                }
            },
            None => Token::EOF,
        }
    }

    /// Sets self.peek to be the next char in self.source_chars
    fn get_char(&mut self) {
        self.peek = self.source_chars.next();
    }

    /// Skips over whitespace in self.source_chars
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

    /// Returns the next number that can be found in self.source_chars
    fn get_num(&mut self) -> i32 {
        let mut s = String::new();
        s.push(self.peek.unwrap());
        loop {
            self.get_char();
            match self.peek {
                Some(c) if c.is_digit(10) => s.push(c),
                _ => break,
            }
        }
        s.parse::<i32>().unwrap()
    }

    /// Returns an Token that contains the next identifier in self.source_chars
    ///
    /// It can be one of three Tokens:
    /// 1. Token::Keyword (if the identifier is a reserved word)
    /// 2. Token::Operator (if the identifier is the name of an operator like `and` or `or`)
    /// 3. Token::Ident (otherwise)
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

    /// Returns a String containing the next symbol spelt operator
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

// Implement Iterator for Scanner
impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    /// Returns the next token as an Option<Token>
    ///
    /// Token::EOF is translated into the end of the iteration
    ///
    /// # Examples
    ///```
    /// # use haumea::scanner::{Scanner, Token};
    /// let s = Scanner::new("1 + 1");
    /// assert_eq!(s.next(), Some(Token::Number(1)));
    /// assert_eq!(s.next(), Some(Token::Operator("+".to_string())));
    /// assert_eq!(s.next(), Some(Token::Number(1)));
    /// assert_eq!(s.next(), None);
    ///```
    fn next(&mut self) -> Option<Token> {
        let tok = self.next_token();
        match tok {
            Token::EOF => None,
            _ => Some(tok),
        }
    }
}
