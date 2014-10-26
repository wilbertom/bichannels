#![crate_name = "bichannels"]

//! # Bidirectional Channels
//!
//! This library implements bidirectional channels in Rust.
//! The language has primitives to do this with the `channel`
//! function. `channel()` returns a `(sender, receiver)` pair
//! which can communicate with each other using message passing.
//! These channels are used to communicate through different
//! [tasks](http://doc.rust-lang.org/guide.html#tasks).
//!
//! A limitation of this is that only one can send messages,
//! and the other can only receive messages.
//! Using Rust's primitives we can extend this behaviour
//! to allow sending and receiving messages from both ends.
//! We call these ends Endpoints.
//!
//! ## Usage
//!
//! ```
//! extern crate bichannels;
//!
//! fn main() {
//!
//!   let bichannels::BiChannel{e1:e1, e2:e2} = bichannels::BiChannel::new();
//!
//!   spawn(proc(){
//!     e2.send("Hello");
//!     let r = e2.recv();
//!     println!("Endpoint 2 got: {}", r);
//!   });
//!
//!   println!("Endpoint 1 got: {}", e1.recv());
//!   e1.send("Oh, hai.");
//!
//! }
//! ```
//!
//! # Unstable
//!
//! This library is unstable because, channels themselves
//! are unstable in Rust.
//!

#[unstable]
pub struct Endpoint<T> {
  sender: Sender<T>,
  receiver: Receiver<T>
}

impl<T: Send> Endpoint<T> {
  pub fn new() -> (Endpoint<T>, Endpoint<T>) {
    let (tx, rx) = channel();
    let (tx2, rx2) = channel();

    (Endpoint {sender: tx2, receiver: rx},
     Endpoint {sender: tx, receiver: rx2})
  }

  pub fn recv(&self) -> T {
    self.receiver.recv()
  }

  pub fn send(&self, t: T) {
    self.sender.send(t);
  }

}

#[unstable]
pub struct BiChannel<T> {
  pub e1: Endpoint<T>,
  pub e2: Endpoint<T>
}

impl<T: Send> BiChannel<T> {

  pub fn new() -> BiChannel<T> {

    let (e1, e2) = Endpoint::new();

    BiChannel {
      e1: e1,
      e2: e2
    }
  }
}
