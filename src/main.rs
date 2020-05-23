use std::io;
use std::env;
// use std::error::Error;
use std::fs;
use csv::Reader;
use csv::ByteRecord;
use std::ffi::OsString;
use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    version: i8,
}


fn transform(record: ByteRecord) -> ByteRecord {
    // Just some modification
    ByteRecord::from(vec![b"test"])
}


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


fn parse_config_from_file(path: OsString) -> Result<Config, String> {
    match fs::read_to_string(path) {
        Ok(content) => match serde_yaml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(err) => Err(format!("Could not parse YAML: {}", err.to_string()))
        },
        Err(err) => Err(format!("Cannot open config: {}", err.to_string()))
    }
}


fn main() {
    let config = match env::args_os().nth(1) {
       None => Err(String::from("Please specify configuration file.")),
       Some(file_path) => parse_config_from_file(file_path),
    };

    println!("config: {:?}", config);
}
