# Bidirectional Channels

This library implements bidirectional channels in Rust.
The language has primitives to do this with the `channel`
function. `channel()` returns a `(sender, receiver)` pair
which can communicate with each other using message passing.
These channels are used to communicate through different
[tasks](http://doc.rust-lang.org/guide.html#tasks).

A limitation of this is that only one can send messages,
and the other can only receive messages.
Using Rust's primitives we can extend this behaviour
to allow sending and receiving messages from both ends.
We call these ends Endpoints.

## Usage

```rust
extern crate bichannels;

fn main() {
let bichannels::BiChannel{e1:e1, e2:e2} = bichannels::BiChannel::new();

spawn(proc(){
e2.send("Hello");
let r = e2.recv();
println!("Endpoint 2 got: {}", r);
});

println!("Endpoint 1 got: {}", e1.recv());
e1.send("Oh, hai.");
}
```
