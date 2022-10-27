# typetag-comparison
Comparing Rust's typetag with a personal implementation

I've been trying to find a good solution to polymporphism + serialization for a specific use case in my Rust LoL clone (basically a RPC thing using polymporphic types). The problem I'm dealing with in particular was originally exactly what the typetag Rust crate solves, but when I compare the output binary data for the two crates, I find that my own solution produces something way smaller (approximately 3x) than what typetag will produce, so I still can't switch to it yet.

I suspect the extra information typetag is including is related to making sure that the type you're deserializing to is 100% the correct type, but what I need for the project is something that allows me to assume that the given data is the right type, so that we don't need to contain all the extra type info inside of the given data.

This is why I'm considering forking typetag with my own implementation that allows you to make assumptions on types like this.

## Usage
To test, run the "client" and then the "server", which emulates the client serializing a command and then the server deserializing it.

#### Use the "typetag" binary to see what the code looks like and see the generated "file.bin".

cargo run --bin typetag --features client

cargo run --bin typetag --features server


#### Use the "my" binary to see what the code looks like from my implementation, and see the smaller generated "file.bin".

cargo run --bin my --features client

cargo run --bin my --features server
