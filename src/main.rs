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
	end

	to main do
	    display(factorial(5))
	end
    ";

   /* let scanner = haumea::scanner::Scanner::new(source);
	for tok in scanner {
		println!("{:?}", tok)
	} */
	let scanner = haumea::scanner::Scanner::new(source);
	let ast = haumea::parser::parse(scanner);
    //println!("{:?}", ast);
	let mut out = String::new();
	haumea::codegen::compile_ast(&mut out, ast);
	println!("{}", out);
}
