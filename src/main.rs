extern crate haumea;

fn main() {
    let scanner = haumea::scanner::Scanner::new("if (1+123)*3 == x then foo()");
    println!("Starting to scan");
    for token in scanner {
        println!("{:?}", token);
    }
}
