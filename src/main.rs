extern crate haumea;

fn main() {
    let source = "
    to bar do
        return 1 + 2 * 3
    end";
    
    let scanner = haumea::scanner::Scanner::new(source);
    println!("{:?}", haumea::parser::parse(scanner));
}
