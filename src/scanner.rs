//! src/scanner.rs
//! The scanner for the haumea language

use std::str::Chars; // We need to bring the Chars struct into scope
use std::iter::Peekable;
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
    source_chars: Peekable<Chars<'a>>,
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
    /// The column the scanner is on in the source
    pub column: u32,
    /// The line the scanner is on in the source
    pub line: u32,
}

/// A structure containing the state of the scanner when it found a token
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct ScanState {
    /// The line the scanner was on
    pub line: u32,
    /// The column the scanner was on
    pub column: u32,
}

impl ScanState {
    /// Constructs a new ScanState
    pub fn new(line: u32, column: u32) -> ScanState {
        ScanState { line: line, column: column }
    }
    /// Constructs an empty ScanState
    pub fn empty() -> ScanState {
        ScanState::new(0, 0)
    }
}

/// An enum representing the various tokens that can occur
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    /// An integer number
    ///
    /// The content is the number read as an i64
    Number(i32, ScanState),
    /// An identifier
    ///
    /// The content is the name of the identifier
    Ident(String, ScanState),
    /// A reserved word (or keyword)
    ///
    /// The content is the name of the keyword
    Keyword(String, ScanState),
    /// An operator
    ///
    /// The content is the name of the operator
    Operator(String, ScanState),
    /// Left parens
    Lp(ScanState),
    /// Right parens
    Rp(ScanState),
    /// A comma
    Comma(ScanState),
    /// An unexpected char was read
    ///
    /// The content is the char read
    Error(char, ScanState),
    /// End of input
    EOF(ScanState),
}

impl Token {
    pub fn state(self) -> ScanState {
        use self::Token::*;
        match self {
            Number(_, s) => s,
            Ident(_, s) => s,
            Keyword(_, s) => s,
            Operator(_, s) => s,
            Error(_, s) => s,
            Lp(s) => s,
            Rp(s) => s,
            Comma(s) => s,
            EOF(s) => s,
        }
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        use self::Token::*;
        match (self, other) {
            (&Number(ref a, _), &Number(ref b, _)) => a == b,
            (&Ident(ref a, _), &Ident(ref b, _)) => a == b,
            (&Keyword(ref a, _), &Keyword(ref b, _)) => a == b,
            (&Operator(ref a, _), &Operator(ref b, _)) => a == b,
            (&Lp(_), &Lp(_)) => true,
            (&Rp(_), &Rp(_)) => true,
            (&Comma(_), &Comma(_)) => true,
            (&Error(ref a, _), &Error(ref b, _)) => a == b,
            (&EOF(_), &EOF(_)) => true,
            _ => false,
        }
    }
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
        let chars = source.chars().peekable();
        let peek = Some(' ');
        Scanner {
            source_str: source,
            source_chars: chars,
            operator_chars: vec!['+', '=', '-', '*', '/', '<', '>', '~', '|', '&', '(', ')', '!'],
            operators: vec!["+", "=", "-", "*", "/", "<", ">", ">=", "<=",
                            "~", "|", "&", "and", "or", "not", "(", ")", "!=", "modulo"],
            ident_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".chars().collect::<Vec<_>>(),
            reserved_words: vec!["to", "with", "is", "return", "do", "end",
                                 "if", "then", "else", "let", "be", "forever",
                                 "while", "for", "each", "in",
                                 "set", "to", "through", "change", "by", "variable"],
            peek: peek,
            column: 0,
            line: 1,
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
        let state = ScanState::new(self.line, self.column);
        match self.peek {
            Some(c) => {
                if self.ident_chars.contains(&c) {
                    self.get_ident_token(state)
                } else if c.is_digit(10) {
                    Token::Number(self.get_num(), state)
                } else if c == '(' {
                    self.get_char();
                    Token::Lp(state)
                } else if c == ')' {
                    self.get_char();
                    Token::Rp(state)
                } else if c == ',' {
                    self.get_char();
                    Token::Comma(state)
                } else if self.operator_chars.contains(&c) {
                    Token::Operator(self.get_op(), state)
                } else {
                    self.get_char();
                    Token::Error(c, state)
                }
            },
            None => Token::EOF(state),
        }
    }

    /// Sets self.peek to be the next char in self.source_chars
    fn get_char(&mut self) {
        self.peek = self.source_chars.next();
        self.column += 1;
        if let Some('\n') = self.peek {
            self.line += 1;
            self.column = 1;
        };
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
        self.skip_comments();
        loop {
            match self.peek {
                Some(c) if c.is_whitespace() => {
                    self.get_char()
                }
                _ => break,
            }
        }
    }
    
    /// Skips over comments in self.source_chars
    fn skip_comments(&mut self) {
        let should_skip =  match self.peek {
            Some(c) if c == '/' => {
                if let Some(n) = self.source_chars.peek() {
                    n == &'*'
                } else {
                    false
                }
            },
            _ => false
        };
        if should_skip {
            self.skip_until_comment_end()
        }
    }
    
    /// Skips until the end of a comment
    fn skip_until_comment_end(&mut self) {
        self.get_char(); // Skip the ? in the start of the comment
        loop {
            self.get_char();
            match self.peek {
                Some(c) if c == '*' => {
                    if let Some(n) = self.source_chars.peek() {
                        if n == &'/' {
                            break;
                        }
                    }
                },
                Some(c) if c == '/' => self.skip_comments(),
                _ => ()
            }
        }
        self.get_char();
        self.get_char();
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
    fn get_ident_token(&mut self, state: ScanState) -> Token {
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
            Token::Keyword(s, state)
        } else if self.operators.contains(&&s[..]) {
            Token::Operator(s, state)
        } else {
            Token::Ident(s, state)
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
            Token::EOF(_) => None,
            _ => Some(tok),
        }
    }
}
