extern crate rand;

use std::thread::{spawn, sleep};
use std::process;
use std::time::Duration;
use rand::{thread_rng,Rng};

/// Wrap your the main function of your application with this macro to
/// have it stop after a random period of time of up to an hour.
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

pub fn nobody_expects() {
    spawn(|| the_spanish_inquisition());
}

fn the_spanish_inquisition() {
    let delay_ms = thread_rng().gen_range(0, 3600 * 1000);
    sleep(Duration::from_millis(delay_ms));
    eprintln!("NOBODY EXPECTS THE SPANISH INQUISITION!");
    process::exit(-1);
}