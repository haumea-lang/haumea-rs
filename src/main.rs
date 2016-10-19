extern crate haumea;

fn main() {
    let source = "
    to factorial with (n is an Integer, returns an Integer) do
        if n = 0 then do
            return 1
        end
        else do
            return n * factorial(n - 1)
        end
    end";
    let scanner = haumea::scanner::Scanner::new(source);
    println!("Starting to scan");
    for token in scanner {
        println!("{:?}", token);
    }
}
