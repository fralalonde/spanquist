#[macro_use] extern crate spanquist;

use std::time;
use std::thread;

spanquist! {
fn main() {
    loop {
        println!("Hello, world!");
        thread::sleep(time::Duration::from_millis(1000));
    }
}
}
