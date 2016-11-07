make:
	@cargo build --release

do: $(file)
	@cargo build
	@./target/debug/haumea < $(file) > out.c
	@cc out.c -o out
	@./out
	@rm out
