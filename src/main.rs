mod options;
mod config;
mod worker;
mod transformer;
mod printable_error;
mod lib;

use std::env;
use crate::lib::run;


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run(args)
}
