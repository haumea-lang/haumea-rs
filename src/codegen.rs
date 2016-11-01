/// codegen.rs
/// The code generator for the haumea language.
use std::rc::Rc;
use parser;

const INDENT: &'static str = "    ";
const NEW_LINE: &'static str = "\n";
const PROLOG: &'static str = "
/* Haumea prolog */
#include <stdio.h>

long display(long n) {
    printf(\"%ld\\n\", n);
    return 0;
}

/* End prolog */

/* Start compiled program */
";
const EPILOG: &'static str = "
/* End compiled program */
";

static mut _name_number: i32 = 0;

/// Compile an Program created by `parser::parse` into a C program
pub fn compile_ast(mut out: &mut String, ast: parser::Program) {
    out.push_str(PROLOG);
    for func in ast {
        compile_function(&mut out, func);
    }
    out.push_str(EPILOG);
}

/// Compiles a Function
fn compile_function(mut out: &mut String, func: parser::Function) {
    write_newline(&mut out);
    out.push_str(if func.name == "main" { "int " } else { "long " });
    out.push_str(&func.name);
    out.push_str("(");
    if let Some(sig) = func.signature {
        if let Some((last_param, first_params)) = sig.split_last() {
            for param in first_params {
                out.push_str(&format!("long {:}, ", param));
            }
            out.push_str(&format!("long {:}", last_param));
        }
    }
    out.push_str(") ");
    out.push_str("{\n");
    compile_statement(&mut out, func.code, 1);
    out.push_str("\n}\n");
}

/// Compiles a statement
fn compile_statement(mut out: &mut String, statement: parser::Statement, indent: i32) {
    use parser::Statement;
    
    match statement {
        Statement::Return(exp) => {
            out.push_str(&format!("{:}return {:};",
                                  replicate(INDENT, indent),
                                  compile_expression(exp)));
        },
        Statement::Do(block) => {
            out.push_str(&format!("\n{:}{{\n", replicate(INDENT, indent)));
            for sub_statement in block {
                let sub = match Rc::try_unwrap(sub_statement) {
                    Ok(sub) => sub,
                    Err(_) => panic!("Could not compile!"),
                };
                compile_statement(&mut out, sub, indent+1);
            };
            out.push_str(&format!("\n{:}}}\n", replicate(INDENT, indent)));
        },
        Statement::Call {
            function: func,
            arguments: args,
        } => {
            out.push_str(&format!("{:}{:}(", replicate(INDENT, indent), func));
            let len = args.len();
            for (index, arg) in args.into_iter().enumerate() {
                if index == len-1 {
                    out.push_str(&compile_expression(arg));
                } else {
                    out.push_str(&format!("{:}, ", compile_expression(arg)));
                }
            }
            out.push_str(");\n");
        },
        Statement::Var(ident) => {
            out.push_str(&format!("{:}long {:};\n", replicate(INDENT, indent), ident));
        },
        Statement::Set(ident, expr) => {
            out.push_str(&format!("{:}{:} = {:};\n",
                                  replicate(INDENT, indent),
                                  ident,
                                  compile_expression(expr)
                              ));
        },
        Statement::Change(ident, expr) => {
            out.push_str(&format!("{:}{:} += {:};\n",
                                  replicate(INDENT, indent),
                                  ident,
                                  compile_expression(expr)
                              ));
        },
        Statement::If {
            cond,
            if_clause,
            else_clause,
        } => {
            out.push_str(&format!("{:}if ", replicate(INDENT, indent)));
            out.push_str(&format!(" {:} ", compile_expression(cond)));
            let if_clause = match Rc::try_unwrap(if_clause) {
                Ok(if_clause) => if_clause,
                Err(_) => panic!("Could not compile!"),
            };
            compile_statement(&mut out, if_clause, indent+1);
            let else_clause = match Rc::try_unwrap(else_clause) {
                Ok(else_clause) => else_clause,
                Err(_) => panic!("Could not compile!"),
            };
            if let Some(else_) = else_clause {
                out.push_str(&format!("{:}else ", replicate(INDENT, indent)));
                compile_statement(&mut out, else_, indent+1);
            }
        },
        Statement::Forever(block) => {
            out.push_str(&format!("{:}while (1) ", replicate(INDENT, indent)));
            let block = match Rc::try_unwrap(block) {
                Ok(block) => block,
                Err(_) => panic!("Could not compile!"),
            };
            compile_statement(&mut out, block, indent+1);
        },
        Statement::While {
            cond,
            body,
        } => {
            out.push_str(&format!("{:}while {:} ", replicate(INDENT, indent),
                                                   compile_expression(cond)));
            let body = match Rc::try_unwrap(body) {
                Ok(body) => body,
                Err(_) => panic!("Could not compile!"),
            };
            compile_statement(&mut out, body, indent+1);
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
                panic!("Could not compile!")
            }
            
            let start_name = get_unique_name();
            let end_name = get_unique_name();
            let by_name = get_unique_name();
            out.push_str(&format!("{:}long {:} = {:};\n",
                                  replicate(INDENT, indent),
                                  start_name,
                                  compile_expression(start)
                              ));
            out.push_str(&format!("{:}long {:} = {:};\n",
                                  replicate(INDENT, indent),
                                  end_name,
                                  compile_expression(end))
                              );
              out.push_str(&format!("{:}long {:} = {:};\n",
                                    replicate(INDENT, indent),
                                    by_name,
                                    compile_expression(by))
                              );
            let comp = format!("({:} < {:} ? {:} {:} {:} : {:} {:} {:})", 
                               start_name, end_name, ident, comparitor, end_name, ident, neg_comparitor, end_name);
            out.push_str(&format!("{:}for (long {:} = {:}; {:}; {:} += {:})", replicate(INDENT, indent),
                                  ident, start_name, comp, ident, by_name
                                  ));
              let body = match Rc::try_unwrap(body) {
                  Ok(body) => body,
                  Err(_) => panic!("Could not compile!"),
              };
            compile_statement(&mut out, body, indent+1);
        },
    }
}

fn compile_expression(expr: parser::Expression) -> String {
    use parser::Expression;
    
    match expr {
        Expression::Integer(i) => format!("{:?}l", i),
        Expression::Ident(name) => name,
        Expression::BinaryOp {
            operator: op,
            left,
            right,
        } => {
            let lh = match Rc::try_unwrap(left) {
                Ok(lh) => lh,
                Err(_) => panic!("Could not compile!"),
            };
            let rh = match Rc::try_unwrap(right) {
                Ok(rh) => rh,
                Err(_) => panic!("Could not compile!"),
            };
            format!("({:} {:} {:})",
                     compile_expression(lh),
                     get_c_name(op),
                     compile_expression(rh)
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
                let arg = match Rc::try_unwrap(arg) {
                    Ok(arg) => arg,
                    Err(_) => panic!("Could not compile!"),
                };
                if index == len-1 {
                    out.push_str(&compile_expression(arg));
                } else {
                    out.push_str(&format!("{:}, ", compile_expression(arg)));
                }
            }
            out.push_str(")");
            out
        },
        Expression::UnaryOp {
            operator: op,
            expression: exp,
        } => {
            let exp = match Rc::try_unwrap(exp) {
                Ok(exp) => exp,
                Err(_) => panic!("Could not compile!"),
            };
            format!("({:}{:})",
                     get_c_name(op),
                     compile_expression(exp)
                   )
        }
    }
}

// Utility functions

/// Writes a newline to out
fn write_newline(mut out: &mut String) {
    out.push_str(NEW_LINE);
}

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
    }
}

/// Returns a unique variable name
fn get_unique_name() -> String {
    unsafe {
        _name_number += 1;
        format!("____HAUMEA_TEMP_{:}", _name_number)
    }
}
