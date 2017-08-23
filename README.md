# spanquist

Abruptly stop an application after a random amount of time.

**_Because NOBODY EXPECTS THE SPANISH INQUISITION_**

[![Build Status](https://travis-ci.org/fralalonde/spanquist.svg?branch=master)](https://travis-ci.org/fralalonde/spanquist/)

# (mis)usage

Add to your `Cargo.toml` the following :
```toml
[dependencies]
spanquist = "*"
```

Then wrap the main function of the application with the `spanquist!` macro.
```rust
#[macro_use] extern crate spanquist;
...
spanquist! {
fn main() {
    loop {
        println!("Hello, world!");
        thread::sleep(time::Duration::from_millis(1000));
    }
}}
```

# try it
To see `spanquist` in action, clone this repo and from the root of it execute 
```sh
cargo run
```
Then, wait up to an hour to see a beautiful application get brutally killed. 

# not my fault
Any hilarity, chaos or straight out murder resulting from the use of this crate is your sole responsibility. Anyway, what are going to do? Bleed on me?
