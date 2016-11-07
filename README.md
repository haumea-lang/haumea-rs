<div align='center'>
  <img src='haumea.png' alt='Haumea'>
</div>

[![Join the chat at https://gitter.im/haumea-lang/Lobby](https://badges.gitter.im/haumea-lang/Lobby.svg)](https://gitter.im/haumea-lang/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) [![Build Status](https://travis-ci.org/haumea-lang/haumea.svg?branch=master)](https://travis-ci.org/haumea-lang/haumea)

Haumea is an experimental language designed to be easy to learn and use.
# Using

First, make sure that you have Rust and Cargo installed. Then, simply clone this repo-

```sh
$ git clone https://github.com/BookOwl/haumea.git
```

-and just use the following to run the Haumea compiler, where `hello.hau` is your Haumea program. (This will compile the Haumea source, if required, compile `hello.hau`, use `cc` to compile the result, and finally run the binary.)

```sh
$ make do file=hello.hau

# or if that doesn't work:

$ cargo build
$ ./target/debug/haumea < hello.hau > out.c
$ cc out.c -o out
$ ./out
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

# Credits
@BookOwl - Created the langauge

@nanalan - Made an amazing logo

Many other people who have helped with design decisions
