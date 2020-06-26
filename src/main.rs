use std::env;
// use crate::lib::run;


extern crate ysv;
use ysv::run;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run(args)
}
