extern crate haumea;
use haumea::{scanner};

fn main() {
    let mut scan = scanner::Scanner::new("1 + 1");
    println!("{:?}", scan.next());
    println!("{:?}", scan.next());
    println!("{:?}", scan.next());
}
