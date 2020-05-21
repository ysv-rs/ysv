use std::io;
// use std::env;
// use std::error::Error;
// use std::ffi::OsString;
// use std::fs::File;
use csv::Reader;


fn pass_through() {
    let mut reader = Reader::from_reader(io::stdin());

    let headers = reader.headers().expect("Where are my headers?!").clone();
    println!("Headers: {:?}", headers);

    for result in reader.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                eprintln!("ERROR reading CSV from <stdin>: {}", err);
            }
        }
    }
}


fn main() {
    // let mut reader = match env::args_os().nth(1) {
    //    None => ,
    //    Some(file_path) => Reader::from_path(file_path).expect("Cannot open a file!"),
    // }

    pass_through();
}
