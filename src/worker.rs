use std::io;
use csv::{ByteRecord, StringRecord, Reader, Writer};
use std::collections::BTreeMap;

use crate::config::{Config, create_transformer};


fn transform(record: ByteRecord, config: &Config, headers: &StringRecord) -> ByteRecord {
    for (name, column) in &config.columns {
        eprintln!("{} -> {:?}", name, column);
    }
    record
}


pub fn process(config: Config) {
    eprintln!("Using config: {:#?}", config);

    let mut reader = Reader::from_reader(io::stdin());
    let mut writer = Writer::from_writer(io::stdout());

    let headers = reader.headers().expect("Where are my headers?!").clone();
    writer.write_record(&headers).expect("woo");

    let transformer = create_transformer(&config, &headers);
    eprintln!("{:?}", transformer);

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
