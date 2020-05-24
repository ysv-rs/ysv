use std::io;
use csv::{ByteRecord, StringRecord, Reader, Writer};
use std::collections::BTreeMap;

use crate::config::{Config, Column};

#[derive(Debug)]
enum Expression {
    Input(usize),
}


#[derive(Debug)]
struct Transformer {
    columns: Vec<Vec<Expression>>,
}


fn get_input_columns_index_map(headers: &StringRecord) -> BTreeMap<String, usize> {
    // FIXME awful function, I do not know the proper method yet
    let mut mapping = BTreeMap::new();

    for (index, value) in headers.iter().enumerate() {
        mapping.insert(String::from(value), index);
    }

    mapping
}


fn create_transformer(config: &Config, headers: &StringRecord) -> Transformer {
    let input_columns_index_by_name = get_input_columns_index_map(headers);
    eprintln!("Index by name: {:?}", input_columns_index_by_name);

    for (output_column_name, column) in config.columns.iter() {
        let column_transformations = match column {
            Column::Input(raw_column_name) => match input_columns_index_by_name.get(
                raw_column_name
            ) {
                Some(index) => Ok(vec![Expression::Input(index.clone())]),
                None => Err(format!("Column {} is not found in the input file.", raw_column_name))
            },
            Column::Steps(transformations) => Ok(vec![])
        };
    }

    Transformer {
        columns: vec![]
    }
}


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
