/// src/parser.rs
/// The parser for the haumea language.
use std::rc::Rc;
use scanner::{Scanner, Token, ScanState};

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum Statement {
    /// A return statement
    ///
    /// return 1
    Return(Expression),
/*    /// A let statement
    ///
    /// let x be an Integer
    Let(Ident, Type), */
    /// A variable statement
    ///
    /// variable x
    Var(Ident),
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
    /// A forever loop
    ///
    /// forever do ... end
    Forever(Rc<Statement>),
    /// A while loop
    ///
    /// while x < 5 change x by 1
    While {
        cond: Expression,
        body: Rc<Statement>,
    },
    /// A for each loop
    ForEach {
        ident: Ident,
        start: Expression,
        end: Expression,
        by: Expression,
        range_type: String,
        body: Rc<Statement>,
    }
}

/// The operators in Haumea
#[derive(Debug, Clone)]
pub enum Operator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Sub,
    /// Multiplication (*)
    Mul,
    /// Division (/)
    Div,
    /// Modulo (modulo)
    Modulo,
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

#[derive(Debug, Clone)]
pub enum Expression {
    /// A binary operation (eg, "1 + 2" or "True or False")
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
    match_panic(&mut token_stream, Token::Keyword("to".to_string(), ScanState::empty()));
    let name = match token_stream.remove(0) {
        Token::Ident(s, _) => s,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
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
    if token_stream[0] == Token::Keyword("with".to_string(), ScanState::empty()) {
        let mut args = vec![];
        match_panic(&mut token_stream, Token::Keyword("with".to_string(), ScanState::empty()));
        match_panic(&mut token_stream, Token::Lp(ScanState::empty()));
        loop {
            args.push(match token_stream.remove(0) {
                Token::Ident(name, _) => name,
                Token::Rp(_) => break,
                t => {
                    let s = t.clone().state();
                    panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
                    s.line, s.column, t)
                },
            });
            if token_stream[0] == Token::Rp(ScanState::empty()) {
                token_stream.remove(0);
                break;
            }
            match_panic(&mut token_stream, Token::Comma(ScanState::empty()));
        }
        Some(args)
    } else {
        None
    }
}

fn parse_statement(mut token_stream: &mut Vec<Token>) -> Statement {
    match token_stream.remove(0) {
        Token::Keyword(t, _) => {
            if t == "return" {
                parse_return(&mut token_stream)
            } else if t == "do" {
                parse_do(&mut token_stream)
            } else if t == "if" {
                parse_if(&mut token_stream)
            } else if t == "set" {
                parse_set(&mut token_stream)
            } else if t == "change" {
                parse_change(&mut token_stream)
            } else if t == "variable" {
                parse_declare(&mut token_stream)
            } else if t == "forever" {
                parse_forever(&mut token_stream)
            } else if t == "while" {
                parse_while(&mut token_stream)
            } else if t == "for" {
                parse_for_each(&mut token_stream)
            } else {
                panic!("Invalid statement!")
            }
        }
        t @ Token::Ident(..) => {
            token_stream.insert(0, t);
            parse_call(&mut token_stream)
        },
        t => {
            let s = t.clone().state();
            panic!("Syntax error at line {:}:{:}, found {:?}", 
            s.line, s.column, t)
        },
    }
}

fn parse_forever(mut token_stream: &mut Vec<Token>) -> Statement {
    Statement::Forever(Rc::new(parse_statement(&mut token_stream)))
}

fn parse_while(mut token_stream: &mut Vec<Token>) -> Statement {
    Statement::While{
        cond: parse_expression(&mut token_stream),
        body: Rc::new(parse_statement(&mut token_stream))
    }
}

fn parse_for_each(mut token_stream: &mut Vec<Token>) -> Statement {
    match_panic(&mut token_stream, Token::Keyword("each".to_string(), ScanState::empty()));
    let ident = match token_stream.remove(0) {
        Token::Ident(name, _) => name,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
    };
    match_panic(&mut token_stream, Token::Keyword("in".to_string(), ScanState::empty()));
    let start = parse_expression(&mut token_stream);
    
    let range_token = token_stream.remove(0);
    let end = parse_expression(&mut token_stream);
    let range_type;
    
    if range_token == Token::Keyword("to".to_string(), ScanState::empty()) {
        range_type = "to";
    } else if range_token == Token::Keyword("through".to_string(), ScanState::empty()) {
        range_type = "through";
    } else {
        let s = range_token.clone().state();
        panic!("At line {:}:{:}, expected 'to' or 'through', not {:?}", s.line, s.column, range_token);
    }
    
    let by = match token_stream[0] {
        Token::Keyword(ref kw, _) => kw == &"by",
        _ => false,
    };
    let by = if by {
        token_stream.remove(0);
        parse_expression(&mut token_stream)
    } else {
        Expression::Integer(1)
    };
    Statement::ForEach {
        ident: ident,
        start: start,
        end: end,
        by: by,
        range_type: range_type.to_string(),
        body: Rc::new(parse_statement(&mut token_stream))
    }
}

fn parse_return(mut token_stream: &mut Vec<Token>) -> Statement {
    Statement::Return(parse_expression(&mut token_stream))
}

fn parse_declare(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident, _) => ident,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
    };
    Statement::Var(ident)
}
fn parse_do(mut token_stream: &mut Vec<Token>) -> Statement {
    let mut block = vec![];
    while token_stream[0] != Token::Keyword("end".to_string(), ScanState::empty()) {
        block.push(Rc::new(parse_statement(&mut token_stream)));
    }
    token_stream.remove(0);
    Statement::Do(block)
}

fn parse_if(mut token_stream: &mut Vec<Token>) -> Statement {
    let cond = parse_expression(&mut token_stream);
    match_panic(&mut token_stream, Token::Keyword("then".to_string(), ScanState::empty()));
    let if_clause = Rc::new(parse_statement(&mut token_stream));
    let else_clause = Rc::new(if !token_stream.is_empty() &&
                                 token_stream[0] == Token::Keyword("else".to_string(), ScanState::empty()) {
        match_panic(&mut token_stream, Token::Keyword("else".to_string(), ScanState::empty()));
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
        Token::Ident(ident, _) => ident,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
    };
    match_panic(&mut token_stream, Token::Keyword("to".to_string(), ScanState::empty()));
    let expr = parse_expression(&mut token_stream);
    Statement::Set(ident, expr)
}

fn parse_change(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident, _) => ident,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
    };
    match_panic(&mut token_stream, Token::Keyword("by".to_string(), ScanState::empty()));
    let expr = parse_expression(&mut token_stream);
    Statement::Change(ident, expr)
}

fn parse_call(mut token_stream: &mut Vec<Token>) -> Statement {
    let ident = match token_stream.remove(0) {
        Token::Ident(ident, _) => ident,
        t => {
            let s = t.clone().state();
            panic!("At line {:}:{:}, expected an identifier, but found {:?}!", 
            s.line, s.column, t)
        },
    };
    match_panic(&mut token_stream, Token::Lp(ScanState::empty()));
    let mut args = vec![];
    if token_stream[0] != Token::Rp(ScanState::empty()) {
        loop {
            args.push(parse_expression(&mut token_stream));
            if token_stream[0] == Token::Rp(ScanState::empty()) {
                token_stream.remove(0);
                break;
            }
            match_panic(&mut token_stream, Token::Comma(ScanState::empty()));
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
    if token_stream[0] == Token::Lp(ScanState::empty()) {
        token_stream.remove(0);
        let exp = parse_expression(&mut token_stream);
        match_panic(&mut token_stream, Token::Rp(ScanState::empty()));
        exp
    } else {
        match token_stream.remove(0) {
            Token::Number(n, _) => Expression::Integer(n),
            Token::Operator(op, s) => {
                if op == "-" {
                    Expression::UnaryOp {
                        operator: Operator::Sub,
                        expression: Rc::new(parse_expression(&mut token_stream))
                    }
                } else {
                    panic!("At line {:}:{:}, expected \"-\", but found {:?}!", 
                           s.line, s.column, op)
                }
            }
            Token::Ident(id, _) => {
                if !token_stream.is_empty() && token_stream[0] == Token::Lp(ScanState::empty()) {
                    match_panic(&mut token_stream, Token::Lp(ScanState::empty()));
                    let mut args = vec![];
                    if token_stream[0] != Token::Rp(ScanState::empty()) {
                        loop {
                            args.push(Rc::new(parse_expression(&mut token_stream)));
                            if token_stream[0] == Token::Rp(ScanState::empty()) {
                                token_stream.remove(0);
                                break;
                            }
                            match_panic(&mut token_stream, Token::Comma(ScanState::empty()));
                        }
                    } else {
                        token_stream.remove(0);
                    }
                    Expression::Call{
                        function: id,
                        arguments: args,
                    }
                } else {
                    Expression::Ident(id)
                }
            },
            t => {
                let s = t.clone().state();
                panic!("At line {:}:{:}, expected an expression, but found {:?}!", 
                s.line, s.column, t)
            },
        }
    }
}

fn prec_1(mut token_stream: &mut Vec<Token>) -> Expression {
    let lh = prec_0(&mut token_stream);
    if !token_stream.is_empty() {
        let op = match token_stream.get(0) {
            Some(&Token::Operator(ref name, _)) => {
                if *name == "*" {
                    Operator::Mul
                } else if *name == "/" {
                    Operator::Div
                } else if *name == "modulo" {
                    Operator::Modulo
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
            Some(&Token::Operator(ref name, _)) => {
                if *name == "+" {
                    Operator::Add
                } else if *name == "-" {
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
            Some(&Token::Operator(ref name, _)) => {
                if *name == ">" {
                    Operator::Gt
                } else if *name == ">=" {
                    Operator::Gte
                } else if *name == "<" {
                    Operator::Lt
                } else if *name == "<=" {
                    Operator::Lte
                } else if *name == "=" {
                    Operator::Equals
                } else if *name == "!=" {
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
            Some(&Token::Operator(ref name, _)) => {
                if *name == "and" {
                    Operator::LogicalAnd
                } else if *name == "or" {
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
