mod config;

use std::io;
use std::env;
use std::fs;
use csv::{Reader, StringRecord};
use csv::ByteRecord;
use std::ffi::OsString;
use serde::Deserialize;
use std::collections::BTreeMap;

use config::{Config, parse_config_from_file};


fn transform(record: ByteRecord, config: &Config, headers: &StringRecord) -> ByteRecord {
    record
}


fn process(config: Config) {
    eprintln!("Using config: {:#?}", config);

    let mut reader = Reader::from_reader(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());

    let headers = reader.headers().expect("Where are my headers?!").clone();
    writer.write_record(&headers).expect("woo");

    for result in reader.byte_records() {
        match result {
            Ok(record) => writer.write_record(&transform(
                record, &config, &headers,
            )).expect("boo!"),
            Err(err) => {
                eprintln!("ERROR reading CSV from <stdin>: {}", err);
            }
        };
    }

    writer.flush().expect("cannot flush!");
}


fn main() {
    match env::args_os().nth(1) {
       None => eprintln!("Please specify configuration file."),
       Some(file_path) => match parse_config_from_file(file_path) {
           Ok(config) => process(config),
           Err(err) => eprintln!("{}", err)
       },
    }
}
