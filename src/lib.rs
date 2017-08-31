//! A Monty Python-inspired crate to abruptly stop an application after a random amount of time.
//! Known uses :
//! - Validate that would-be resilient systems properly handle spuriously terminating applications.
//! - Take the piss out of your _upper class twit of the year_ boss, colleagues and customers.

extern crate rand;

use std::thread;
use std::process;
use std::time::Duration;
use rand::{thread_rng,Rng};

/// Randomly schedule self-termination of application.
///
/// `spanquist!` is used by wrapping the `fn main() {}` of the target application.
/// Random termination delay is determined upon start.
/// Use of `spanquist!` is transparent to the application and can not be controlled at runtime.
///
///
/// Upon termination the message `NOBODY EXPECTS THE SPANISH INQUISITION!` is printed to stderr.
/// The delay for termination is currently limited to an hour but could be changed in future
/// releases so as to keep you on your toes.
///
/// ```rust,no_run
/// #[macro_use] extern crate spanquist;
/// use std::time;
/// use std::thread;
///
/// spanquist! {
/// fn main() {
///     loop {
///         println!("Hello, world!");
///         thread::sleep(time::Duration::from_millis(1000));
///     }
/// }}
/// ```
///
/// Any resulting hilarity, chaos or straight out murder is your sole responsibility.
#[macro_export]
macro_rules! spanquist {
    (fn main() $body: block) => {
    fn main() {
        $crate::nobody_expects();
        { $body }
    }}
}

#[allow(unused_must_use)]
pub fn nobody_expects() {
    thread::Builder::new()
        .name("spanquist".to_string())
        .spawn(|| the_spanish_inquisition());
}

fn the_spanish_inquisition() {
    let delay_ms = thread_rng().gen_range(0, 3600 * 1000);
    thread::sleep(Duration::from_millis(delay_ms));
    eprintln!("NOBODY EXPECTS THE SPANISH INQUISITION");
    process::exit(-1);
}