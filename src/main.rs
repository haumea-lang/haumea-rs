extern crate haumea;
use haumea::{scanner};

fn main() {
    let mut scan = scanner::Scanner::new("if (1+123)*3 == x then foo()");
    println!("Starting to scan");
    loop {
        let tok = scan.next();
        match tok {
            scanner::Token::EOF => break,
            _ => println!("{:?}", tok),
        }
    }
}
