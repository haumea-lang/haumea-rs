/// src/parser.rs
/// The parser for the haumea language.
use std::rc::Rc;
use scanner::{Scanner, Token};

/// A Program is a Vec of Functions
pub type Program = Vec<Function>;

/// A Block is a Vec of Rc<Statement>s
pub type Block = Vec<Rc<Statement>>;

/// A Type is a String (for now)
pub type Type = String;

/// An Ident is a String
pub type Ident = String;

/// A Signature is a Vec of Strings
pub type Signature = Vec<String>;

/// A function is a callable unit of code that returns a value
#[derive(Debug)]
pub struct Function {
    /// The name of the function
    name: String,
    /// The signature of the function
    ///
    /// It is a Some(Signature) when there is a signature,
    /// or None if there is no signature, which means that
    /// the function takes no arguments and return the Integer 0
    signature: Option<Signature>,
    /// The code of the function
    code: Statement,
}

/// A Haumea statement
#[derive(Debug)]
pub enum Statement {
    /// A return statement
    ///
    /// return 1
    Return(Expression),
    /// A let statement
    ///
    /// let x be an Integer
    Let(Ident, Type),
    /// An assignment statement
    ///
    /// set x to 5
    Set(Ident, Expression),
    /// A change statement
    ///
    /// change x by -2
    Change(Ident, Expression),
    /// An if statement
    ///
    /// if True then return 1
    /// else return -3
    /// (else is optional)
    If {
        cond: Expression,
        if_clause: Rc<Statement>,
        else_clause: Rc<Option<Statement>>,
    },
    /// A Do statement
    ///
    /// do
    ///   statement1
    ///   statement2
    /// end
    Do(Block),
    /// A call statment
    ///
    /// write_ln(1)
    Call {
        function: Ident,
        arguments: Vec<Expression>,
    },
}

/// The operators in Haumea
#[derive(Debug)]
pub enum Operator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Sub,
    /// Multiplication (*)
    Mul,
    /// Division (/)
    Div,
    /// Negation (-)
    Negate,
    /// Equals (=)
    Equals,
    /// Greater than (>)
    Gt,
    /// Lesser than (<)
    Lt,
    /// Greater than or equal to (>=)
    Gte,
    /// Lesser than or equal to (<=)
    Lte,
    /// Logical And (and)
    LogicalAnd,
    /// Logical OR (or)
    LogicalOr,
    /// Logical Not (not)
    LogicalNot,
    /// Binary And (&)
    BinaryAnd,
    /// Binary Or (|)
    BinaryOr,
    /// Binary Not (~)
    BinaryNot,
}

#[derive(Debug)]
pub enum Expression {
    /// A binary operation (eg, "1 +2" or "True or False")
    BinaryOp {
        operator: Operator,
        left: Rc<Expression>,
        right: Rc<Expression>,
    },
    /// A unary operation (eg, "not False" or "-(1 + 2)")
    UnaryOp {
        operator: Operator,
        expression: Rc<Expression>,
    },
    /// An integer literal
    Integer(i32),
    /// An identifier
    Ident(Ident),
    /// A function call
    Call {
        function: Ident,
        arguments: Vec<Rc<Expression>>,
    },
}

pub fn parse(scanner: Scanner) -> Program {
    let mut tokens = scanner.collect::<Vec<_>>();
    let mut program = vec![];
    while !tokens.is_empty() {
        program.push(parse_function(&mut tokens));
    }
    program
}

fn match_token(mut token_stream: &mut Vec<Token>, expected: &Token) -> Result<Token, Token> {
    let t = token_stream.remove(0);
    if t == *expected {
        Ok(t)
    } else {
        Err(t)
    }
}

fn match_panic(mut token_stream: &mut Vec<Token>, expected: Token) {
    match match_token(&mut token_stream, &expected) {
        Ok(_) => (),
        Err(t) => panic!(format!("Expected {:?}, but found {:?}!", expected, t)),
    }
}

fn parse_function(mut token_stream: &mut Vec<Token>) -> Function {
    match_panic(&mut token_stream, Token::Keyword("to".to_string()));
    let name = match token_stream.remove(0) {
        Token::Ident(s) => s,
        t @ _ => panic!(format!("Expected an identifier, but found {:?}!", t)),
    };
    let signature = parse_signature(&mut token_stream);
    let code = parse_statement(&mut token_stream);
    Function {
               name: name,
               signature: signature,
               code: code,
             }
}

fn parse_signature(mut token_stream: &mut Vec<Token>) -> Option<Signature> {
    if token_stream[0] == Token::Keyword("with".to_string()) {
        let mut args = vec![];
        match_panic(&mut token_stream, Token::Keyword("with".to_string()));
        match_panic(&mut token_stream, Token::Lp);
        loop {
            args.push(match token_stream.remove(0) {
                Token::Ident(name) => name,
                Token::Rp => break,
                t @ _ => panic!(format!("Expected an identifier, but found {:?}!", t)),
            });
            match_panic(&mut token_stream, Token::Comma);
        }
        Some(args)
    } else {
        None
    }
}

fn parse_statement(mut token_stream: &mut Vec<Token>) -> Statement {
    match_panic(&mut token_stream, Token::Ident("foo".to_string()));
    Statement::Return(Expression::Integer(1))
}
