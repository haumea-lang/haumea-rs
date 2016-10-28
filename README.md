<align ='center'>
  <img src='haumea.png' alt='Haumea'>
</align>

[![Join the chat at https://gitter.im/haumea-lang/Lobby](https://badges.gitter.im/haumea-lang/Lobby.svg)](https://gitter.im/haumea-lang/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Haumea is an experimental language created by BookOwl to learn Rust, parsing, and code generation.

# Using

First, make sure that you have Rust and Cargo installed. Then run the following commands:

```
$ git clone https://github.com/BookOwl/haumea.git
$ cargo build
```

To run the compiler, use the following commands:

```
# Compile the .hau file to a .c
$ ./target/debug/haumea < in.hau > out.c
# Compile the .c
$ gcc out.c -o out
```

# Example programs

Here is an example program that calculates factorials:

```
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
```

# Find a bug, or want to request a feature?
Please create an issue with your bug report or pull request.

# Haumea reference.
Please check out the wiki for the Haumea reference and a tutorial.

# License
Haumea is released under the MIT license. Please see LICENSE.txt for details.
