# spanquist [![Build Status](https://travis-ci.org/fralalonde/spanquist.svg?branch=master)](https://travis-ci.org/fralalonde/spanquist/)
A Monty Python-inspired macro that abruptly stops an application after a random amount of time.

# why
`spanquist` can be used as an agent of controlled chaos to validate that a would-be resilient system properly handles spuriously terminating applications.
Because it emits the easily recognizable string 
```
NOBODY EXPECTS THE SPANISH INQUISITION
```
upon killing its host process, such terminations can still be identified, filtered out of logs, etc.

It can also be used to just take the piss out of your twit-of-the-year colleagues.

# (mis)usage
Add to your `Cargo.toml` the following :
```toml
[dependencies]
spanquist = "*"
```

Then wrap the main function of the application with the `spanquist!` macro.
```rust
#[macro_use] extern crate spanquist;

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
Then, wait up to an hour to see a beautiful, fragile application get brutally crushed. 

# not my fault
Any hilarity, chaos or straight out murder resulting from the use of this crate is your sole responsibility. Anyway, what are going to do? Bleed on me?
