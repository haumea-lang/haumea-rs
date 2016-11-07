extern crate haumea;
use std::io;
use std::io::prelude::*;

// Load the CodeGen trait into scope
use haumea::codegen::CodeGen;

fn main() {
    let mut source = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut source).expect("Must provide input");
    let scanner = haumea::scanner::Scanner::new(&source);
    let ast = haumea::parser::parse(scanner);
    let mut cg = haumea::codegen::c::CodeGenerator::new(ast);
    let out = cg.compile();
    println!("{}", out);
}
