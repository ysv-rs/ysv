use std::io;
use csv::{ByteRecord, Reader, Writer, ReaderBuilder};

use crate::config::{Config, create_transformer};
use crate::transformer::{Transformer, Expression};
use std::collections::HashMap;


fn apply_column(column: &Vec<Expression>, record: &ByteRecord) -> String {
    let mut value: Option<String> = None;

    for expression in column.iter() {
        value = expression.apply(value, record);
    }

    match value {
        Some(content) => content,
        None => String::new(),
    }
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


pub fn process(config: Config, variables: HashMap<String, String>) -> Result<(), String> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(io::stdin());

    let mut writer = Writer::from_writer(io::stdout());

    let headers = reader.headers().unwrap().clone();

    let transformer = create_transformer(&config, &headers, &variables)?;

    writer.write_record(&transformer.headers).unwrap();

    for result in reader.byte_records() {
        let record = result.unwrap();

        writer.write_record(&transform(
            record,
            &transformer,
        )).unwrap();
    }

    Ok(writer.flush().unwrap())
}
