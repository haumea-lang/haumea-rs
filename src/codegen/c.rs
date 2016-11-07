//! c.rs
//! The C code generator for the haumea language.
use std::rc::Rc;
use parser;
use codegen;

/// Unwraps a Rc or panics if it is not possible to do so.
/// This is a macro because it needs to not take a reference to the passed in Rc,
/// which is what would happen if it was a function.
macro_rules! unwrap_rc {
   ( $rc:expr ) => ( (*Rc::make_mut(&mut ($rc).clone())).clone() );
   //                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   //           This is probably the ugliest line of Rust I've ever written. :P
}

pub struct CodeGenerator<'a> {
    indent: &'a str,
    prolog: &'a str,
    epilog: &'a str,
    ast: parser::Program,
    _name_number: u32,
    out: String,
}

impl<'a> codegen::CodeGen for CodeGenerator<'a> {
    /// Compile an Program created by `parser::parse` into a C program
    fn compile(&mut self) -> String {
        self.out.push_str(self.prolog);
        for func in self.ast.clone().into_iter() {
            self.compile_function(func);
        }
        self.out.push_str(self.epilog);
        self.out.clone()
    }
}

impl<'a> CodeGenerator<'a> {
    /// Constructs a new CodeGenerator
    pub fn new(ast: parser::Program) -> CodeGenerator<'a> {
        CodeGenerator {
            indent: "    ",
            prolog: "
/* Haumea prolog */
#include <stdio.h>

long display(long n) {
    printf(\"%ld\\n\", n);
    return 0;
}

long read() {
    printf(\"Enter an integer: \");
    long n;
    scanf(\"%ld\", &n);
    return n;
}

/* End prolog */

/* Start compiled program */
",          
            epilog: "
/* End compiled program */
",
            ast: ast,
            _name_number: 0,
            out: String::new(),
        }
    }
    
    /// Compiles a Function
    fn compile_function(&mut self, func: parser::Function) {
        self.out.push_str("\n");
        self.out.push_str(if func.name == "main" { "int " } else { "long " });
        self.out.push_str(&func.name);
        self.out.push_str("(");
        if let Some(sig) = func.signature {
            if let Some((last_param, first_params)) = sig.split_last() {
                for param in first_params {
                    self.out.push_str(&format!("long {:}, ", param));
                }
                self.out.push_str(&format!("long {:}", last_param));
            }
        }
        self.out.push_str(") ");
        self.out.push_str("{\n");
        self.compile_statement(func.code, 1);
        self.out.push_str(&format!("{:}return 0l;", self.indent));
        self.out.push_str("\n}\n");
    }
    
    /// Compiles a statement
    fn compile_statement(&mut self, statement: parser::Statement, indent: i32) {
        use parser::Statement;
    
        match statement {
            Statement::Return(exp) => {
                let exp = self.compile_expression(exp);
                self.out.push_str(&format!("{:}return {:};",
                                      replicate(self.indent, indent),
                                      exp));
            },
            Statement::Do(block) => {
                self.out.push_str(&format!("{:}{{\n", replicate(self.indent, indent)));
                for sub_statement in block {
                    let sub = unwrap_rc!(sub_statement);
                    self.compile_statement(sub, indent+1);
                };
                self.out.push_str(&format!("{:}}}\n", replicate(self.indent, indent)));
            },
            Statement::Call {
                function: func,
                arguments: args,
            } => {
                self.out.push_str(&format!("{:}{:}(", replicate(self.indent, indent), func));
                let len = args.len();
                for (index, arg) in args.into_iter().enumerate() {
                    let arg = self.compile_expression(arg);
                    if index == len-1 {
                        self.out.push_str(&arg);
                    } else {
                        self.out.push_str(&format!("{:}, ", arg));
                    }
                }
                self.out.push_str(");\n");
            },
            Statement::Var(ident) => {
                self.out.push_str(&format!("{:}long {:};\n", replicate(self.indent, indent), ident));
            },
            Statement::Set(ident, expr) => {
                let expr = self.compile_expression(expr);
                self.out.push_str(&format!("{:}{:} = {:};\n",
                                      replicate(self.indent, indent),
                                      ident,
                                      expr
                                  ));
            },
            Statement::Change(ident, expr) => {
                let expr = self.compile_expression(expr);
                self.out.push_str(&format!("{:}{:} += {:};\n",
                                      replicate(self.indent, indent),
                                      ident,
                                      expr
                                  ));
            },
            Statement::If {
                cond,
                if_clause,
                else_clause,
            } => {
                let cond = self.compile_expression(cond);
                self.out.push_str(&format!("{:}if {:}\n", replicate(self.indent, indent), cond));
                let if_clause = unwrap_rc!(if_clause);
                self.compile_statement(if_clause, indent+1);
                let else_clause = unwrap_rc!(else_clause);
                if let Some(else_) = else_clause {
                    self.out.push_str(&format!("\n{:}else\n", replicate(self.indent, indent)));
                    self.compile_statement(else_, indent+1);
                    self.out.push_str("\n");
                }
            },
            Statement::Forever(block) => {
                self.out.push_str(&format!("{:}while (1)\n", replicate(self.indent, indent)));
                let block = unwrap_rc!(block);
                self.compile_statement(block, indent+1);
            },
            Statement::While {
                cond,
                body,
            } => {
                let cond = self.compile_expression(cond);
                self.out.push_str(&format!("{:}while {:}\n", replicate(self.indent, indent),
                                           cond));
                let body = unwrap_rc!(body);
                self.compile_statement(body, indent+1);
            },
            Statement::ForEach {
                ident,
                start,
                end,
                by,
                range_type,
                body,
            } => {
                let comparitor;
                let neg_comparitor;
                if range_type == "to" {
                    comparitor = "<";
                    neg_comparitor = ">";
                } else if range_type == "through" {
                    comparitor = "<=";
                    neg_comparitor = ">=";
                } else {
                    panic!("Invalid range type {:?}!", range_type)
                }
            
                let start_name = self.get_unique_name();
                let end_name = self.get_unique_name();
                let by_name = self.get_unique_name();
                
                let start = self.compile_expression(start);
                self.out.push_str(&format!("{:}long {:} = {:};\n",
                                      replicate(self.indent, indent),
                                      start_name,
                                      start,
                                  ));
                let end = self.compile_expression(end);
                self.out.push_str(&format!("{:}long {:} = {:};\n",
                                      replicate(self.indent, indent),
                                      end_name,
                                      end)
                                  );
                let by = self.compile_expression(by);
                self.out.push_str(&format!("{:}long {:} = {:};\n",
                                      replicate(self.indent, indent),
                                      by_name,
                                      by)
                                  );
                let comp = format!("({:} < {:} ? {:} {:} {:} : {:} {:} {:})", 
                                   start_name, end_name, ident, comparitor, end_name, ident, neg_comparitor, end_name);
                self.out.push_str(&format!("{:}for (long {:} = {:}; {:}; {:} += {:})\n", replicate(self.indent, indent),
                                      ident, start_name, comp, ident, by_name
                                      ));
                let body = unwrap_rc!(body);
                self.compile_statement(body, indent+1);
            },
        }
    }
    
    /// Compiles an expression
    fn compile_expression(&self, expr: parser::Expression) -> String {
        use parser::Expression;
    
        match expr {
            Expression::Integer(i) => format!("{:?}l", i),
            Expression::Ident(name) => name,
            Expression::BinaryOp {
                operator: op,
                left,
                right,
            } => {
                let lh = unwrap_rc!(left);
                let rh = unwrap_rc!(right);
                format!("({:} {:} {:})",
                         self.compile_expression(lh),
                         get_c_name(op),
                         self.compile_expression(rh)
                       )
            },
            Expression::Call {
                function: func,
                arguments: args,
            } => {
                let mut out = String::new();
                out.push_str(&format!("{:}(", func));
                let len = args.len();
                for (index, arg) in args.into_iter().enumerate() {
                    let arg = unwrap_rc!(arg);
                    if index == len-1 {
                        out.push_str(&self.compile_expression(arg));
                    } else {
                        out.push_str(&format!("{:}, ", self.compile_expression(arg)));
                    }
                }
                out.push_str(")");
                out
            },
            Expression::UnaryOp {
                operator: op,
                expression: exp,
            } => {
                let exp = unwrap_rc!(exp);
                format!("({:}{:})",
                         get_c_name(op),
                         self.compile_expression(exp)
                       )
            }
        }
    }
    
    /// Returns a unique name
    fn get_unique_name(&mut self) -> String {
        self._name_number += 1;
        format!("__HAUMEA_TEMP_{:}", self._name_number)
    }
}

// Utility functions

/// Replicates a &str t times
fn replicate(s: &str, t: i32) -> String {
    if t == 0 {
        "".to_string()
    } else {
        replicate(s, t-1) + s
    }
}

/// Returns the C name of an operator
fn get_c_name(op: parser::Operator) -> &'static str {
    use parser::Operator::*;
    match op {
        Add => "+",
        Sub | Negate => "-",
        Mul => "*",
        Div => "/",
        Equals => "==",
        NotEquals => "!=",
        Gt => ">",
        Lt => "<",
        Gte => ">=",
        Lte => "<=",
        LogicalAnd => "&&",
        LogicalOr => "||",
        LogicalNot => "!",
        BinaryAnd => "&",
        BinaryOr => "|",
        BinaryNot => "~",
        Modulo => "%",
    }
}
