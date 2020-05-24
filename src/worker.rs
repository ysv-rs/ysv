use std::io;
use csv::{ByteRecord, StringRecord, Reader, Writer};
use std::collections::BTreeMap;

use crate::config::{Config, create_transformer};
use crate::transformer::{Transformer, Expression};


fn apply_column(column: &Vec<Expression>, record: &ByteRecord) -> String {
    String::from("foo")
}


fn transform(record: ByteRecord, transformer: &Transformer) -> ByteRecord {
    let output: Vec<String> = transformer.columns.iter().map(
        |column| apply_column(
            column,
            &record
        )
    ).collect();

    ByteRecord::from(output)
}


pub fn process(config: Config) {
    eprintln!("Using config: {:#?}", config);

    let mut reader = Reader::from_reader(io::stdin());
    let mut writer = Writer::from_writer(io::stdout());

    let headers = reader.headers().expect("Where are my headers?!").clone();

    match create_transformer(&config, &headers) {
        Ok(transformer) => {
            eprintln!("Using transformer: {:#?}", transformer);
            writer.write_record(&transformer.headers).expect("woo");

            for result in reader.byte_records() {
                match result {
                    Ok(record) => writer.write_record(&transform(
                        record, &transformer,
                    )).expect("boo!"),
                    Err(err) => {
                        eprintln!("ERROR reading CSV from <stdin>: {}", err);
                    }
                };
            }
        },
        Err(err) => eprintln!("Cannot apply config: {}", err)
    }

    writer.flush().expect("cannot flush!");
}
