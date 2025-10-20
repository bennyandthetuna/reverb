# Reverb

an implementation of the echo command from coreutils in Rust 

I did this for fun in my spare time

key difference waiting to be implemented is finishing implementing all of the escape sequence replacements

after I did this I was happy to see that gnu coreutils and I vaguely share the same approach. thought I can probably guarantee they did it better.


## Build

building can be done via the rust built in cargo build

## Run

running reverb can be done two different ways.

1) using the build in cargo run, keep in mind that passing arguments requires a "--" be added after cargo run

ex. cargo run -- e hello world \a

2) calling the binary directly which is put into the target folder upon building.