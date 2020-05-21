use std::io;
// use std::env;
// use std::error::Error;
// use std::ffi::OsString;
// use std::fs::File;
use csv::Reader;


fn pass_through() {
    let mut reader = Reader::from_reader(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());

    let headers = reader.headers().expect("Where are my headers?!").clone();
    writer.write_record(&headers).expect("woo");

    for result in reader.byte_records() {
        match result {
            Ok(record) => writer.write_record(&record).expect("boo!"),
            Err(err) => {
                eprintln!("ERROR reading CSV from <stdin>: {}", err);
            }
        };
    }

    writer.flush().expect("cannot flush!");
}


fn main() {
    // let mut reader = match env::args_os().nth(1) {
    //    None => ,
    //    Some(file_path) => Reader::from_path(file_path).expect("Cannot open a file!"),
    // }

    pass_through();
}
