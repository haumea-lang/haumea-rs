//! Tests for `haumea::parser`
extern crate haumea;

use std::rc::Rc;

use haumea::scanner::*;
use haumea::parser::*;
use haumea::parser::Statement::*;
use haumea::parser::Operator::*;
use haumea::parser::Expression::*;

fn assert_parsed_is(source: &str, expected: Vec<Function>) {
    let scanner = Scanner::new(&source);
    let ast: Vec<Function> = parse(scanner);
    assert_eq!(ast, expected);
}

#[test]
fn test_display_addition() {
    let hello_world_code = "to main do
        display(1+2)
    end";

    let expected_ast = vec![
        Function {
            name: "main".to_string(),
            signature: None,
            code: Do(vec![
                Rc::new(Statement::Call {
                    function: "display".to_string(),
                    arguments: vec![
                        BinaryOp {
                            operator: Add,
                            left: Rc::new(Integer(1)),
                            right: Rc::new(Integer(2))
                        }
                    ]
                })
            ])
        }
    ];

    assert_parsed_is(&hello_world_code, expected_ast);
}
