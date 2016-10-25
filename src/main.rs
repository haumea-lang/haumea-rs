extern crate haumea;

fn main() {
    let source = "
    to factorial with (n) do
        if n = 0 then do
            return 1
        end
        else do
            return n * factorial(n - 1)
        end
    end";

    let scanner = haumea::scanner::Scanner::new(source);
    for tok in scanner {
        println!("{:?}", tok);
    }
    let scanner = haumea::scanner::Scanner::new(source);
    println!("{:?}", haumea::parser::parse(scanner));
}
