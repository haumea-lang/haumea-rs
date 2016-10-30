extern crate haumea;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut source = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut source).expect("Must provide input");
    let scanner = haumea::scanner::Scanner::new(&source);
    let ast = haumea::parser::parse(scanner);
    //println!("{:?}", ast);
    let mut out = String::new();
    haumea::codegen::compile_ast(&mut out, ast);
    println!("{}", out);
}
