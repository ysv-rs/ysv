extern crate ysv;

use std::env;
use ysv::run;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run(args)
}
