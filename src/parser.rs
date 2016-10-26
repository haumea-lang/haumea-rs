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
    pub name: String,
    /// The signature of the function
    ///
    /// It is a Some(Signature) when there is a signature,
    /// or None if there is no signature, which means that
    /// the function takes no arguments and return the Integer 0
    pub signature: Option<Signature>,
    /// The code of the function
    pub code: Statement,
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
    ///Not equals (!=)
    NotEquals,
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
            if token_stream[0] == Token::Rp {
                token_stream.remove(0);
                break;
            }
            match_panic(&mut token_stream, Token::Comma);
        }
        Some(args)
    } else {
        None
    }
}

fn parse_statement(mut token_stream: &mut Vec<Token>) -> Statement {
    match token_stream.remove(0) {
        Token::Keyword(t) => {
            if t == "return".to_string() {
                parse_return(&mut token_stream)
            } else if t == "do".to_string() {
                parse_do(&mut token_stream)
            } else if t == "if".to_string() {
                parse_if(&mut token_stream)
            } else if t == "set".to_string() {
                parse_set(&mut token_stream)
            } else if t == "change".to_string() {
                parse_change(&mut token_stream)
            } else {
                panic!("Invalid statement!")
            }
        }
        t @ Token::Ident(_) => {
            token_stream.insert(0, t);
            parse_call(&mut token_stream)
        },
        t @ _ => panic!("Syntax error! {:?}", t),
    }
    /*
    match_panic(&mut token_stream, Token::Ident("foo".to_string()));
    Statement::Return(Expression::Integer(1))*/
}

fn parse_return(mut token_stream: &mut Vec<Token>) -> Statement {
    Statement::Return(parse_expression(&mut token_stream))
}

fn parse_do(mut token_stream: &mut Vec<Token>) -> Statement {
    let mut block = vec![];
    while token_stream[0] != Token::Keyword("end".to_string()) {
        block.push(Rc::new(parse_statement(&mut token_stream)));
    }
    token_stream.remove(0);
    Statement::Do(block)
}

fn parse_if(mut token_stream: &mut Vec<Token>) -> Statement {
    let cond = parse_expression(&mut token_stream);
    match_panic(&mut token_stream, Token::Keyword("then".to_string()));
    let if_clause = Rc::new(parse_statement(&mut token_stream));
    let else_clause = Rc::new(if !token_stream.is_empty() &&
                                 token_stream[0] == Token::Keyword("else".to_string()) {
        match_panic(&mut token_stream, Token::Keyword("else".to_string()));
        Some(parse_statement(&mut token_stream))
    } else {
        None
    });
    Statement::If {
        cond: cond,
        if_clause: if_clause,
        else_clause: else_clause,
    }
}

fn parse_set(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident) => ident,
        t @ _ => panic!(format!("Expected an identifier, but found {:?}!", t)),
    };
    match_panic(&mut token_stream, Token::Keyword("to".to_string()));
    let expr = parse_expression(&mut token_stream);
    Statement::Set(ident, expr)
}

fn parse_change(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident) => ident,
        t @ _ => panic!(format!("Expected an identifier, but found {:?}!", t)),
    };
    match_panic(&mut token_stream, Token::Keyword("by".to_string()));
    let expr = parse_expression(&mut token_stream);
    Statement::Change(ident, expr)
}

fn parse_call(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident) => ident,
        t @ _ => panic!(format!("Expected an identifier, but found {:?}!", t)),
    };
    match_panic(&mut token_stream, Token::Lp);
    let mut args = vec![];
    if token_stream[0] != Token::Rp {
        loop {
            args.push(parse_expression(&mut token_stream));
            if token_stream[0] == Token::Rp {
                token_stream.remove(0);
                break;
            }
            match_panic(&mut token_stream, Token::Comma);
        }
    }
    Statement::Call{
        function: ident,
        arguments: args,
    }
}

fn parse_expression(mut token_stream: &mut Vec<Token>) -> Expression {
    prec_4(&mut token_stream)
}

fn prec_0(mut token_stream: &mut Vec<Token>) -> Expression {
    if token_stream[0] == Token::Lp {
        token_stream.remove(0);
        let exp = parse_expression(&mut token_stream);
        match_panic(&mut token_stream, Token::Rp);
        exp
    } else {
        match token_stream.remove(0) {
            Token::Number(n) => Expression::Integer(n),
            Token::Ident(id) => {
                if token_stream[0] == Token::Lp {
                    match_panic(&mut token_stream, Token::Lp);
                    let mut args = vec![];
                    if token_stream[0] != Token::Rp {
                        loop {
                            args.push(Rc::new(parse_expression(&mut token_stream)));
                            if token_stream[0] == Token::Rp {
                                token_stream.remove(0);
                                break;
                            }
                            match_panic(&mut token_stream, Token::Comma);
                        }
                    }
                    Expression::Call{
                        function: id,
                        arguments: args,
                    }
                } else {
                    Expression::Ident(id)
                }
            },
            t @ _ => panic!(format!("Expected an expression, not {:?}", t)),
        }
    }
}

fn prec_1(mut token_stream: &mut Vec<Token>) -> Expression {
    let lh = prec_0(&mut token_stream);
    if !token_stream.is_empty() {
        let op = match token_stream.get(0) {
            Some(&Token::Operator(ref name)) => {
                if *name == "*".to_string() {
                    Operator::Mul
                } else if *name == "/".to_string() {
                    Operator::Div
                } else {
                    return lh
                }
            },
            _ => return lh,
        };
        token_stream.remove(0);
        let rh = prec_1(&mut token_stream);
        Expression::BinaryOp {
            operator: op,
            left: Rc::new(lh),
            right: Rc::new(rh),
        }
    } else {
        lh
    }
}

fn prec_2(mut token_stream: &mut Vec<Token>) -> Expression {
    let lh = prec_1(&mut token_stream);
    if !token_stream.is_empty() {
        let op = match token_stream.get(0) {
            Some(&Token::Operator(ref name)) => {
                if *name == "+".to_string() {
                    Operator::Add
                } else if *name == "-".to_string() {
                    Operator::Sub
                } else {
                    return lh
                }
            },
            _ => return lh,
        };
        token_stream.remove(0);
        let rh = prec_2(&mut token_stream);
        Expression::BinaryOp {
            operator: op,
            left: Rc::new(lh),
            right: Rc::new(rh),
        }
    } else {
        lh
    }
}

fn prec_3(mut token_stream: &mut Vec<Token>) -> Expression {
    let lh = prec_2(&mut token_stream);
    if !token_stream.is_empty() {
        let op = match token_stream.get(0) {
            Some(&Token::Operator(ref name)) => {
                if *name == ">".to_string() {
                    Operator::Gt
                } else if *name == ">=".to_string() {
                    Operator::Gte
                } else if *name == "<".to_string() {
                    Operator::Lt
                } else if *name == "<=".to_string() {
                    Operator::Lte
                } else if *name == "=".to_string() {
                    Operator::Equals
                } else if *name == "!=".to_string() {
                    Operator::NotEquals
                } else {
                    return lh
                }
            },
            _ => return lh
        };
        token_stream.remove(0);
        let rh = prec_3(&mut token_stream);
        Expression::BinaryOp {
            operator: op,
            left: Rc::new(lh),
            right: Rc::new(rh),
        }
    } else {
        lh
    }
}

fn prec_4(mut token_stream: &mut Vec<Token>) -> Expression {
    let lh = prec_3(&mut token_stream);
    if !token_stream.is_empty() {
        let op = match token_stream.get(0) {
            Some(&Token::Operator(ref name)) => {
                if *name == "and".to_string() {
                    Operator::LogicalAnd
                } else if *name == "or".to_string() {
                    Operator::LogicalOr
                } else {
                    return lh
                }
            },
            _ => return lh
        };
        token_stream.remove(0);
        let rh = prec_4(&mut token_stream);
        Expression::BinaryOp {
            operator: op,
            left: Rc::new(lh),
            right: Rc::new(rh),
        }
    } else {
        lh
    }
}
