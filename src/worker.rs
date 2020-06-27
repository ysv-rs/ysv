use std::io;
use csv::{ByteRecord, Writer, ReaderBuilder};

use crate::config::create_transformer;
use crate::transformer::{Transformer, Transformation};
use crate::options::{Options, Variables};


type TransformationsChain = Vec<Transformation>;


fn apply_transformations_chain(
    transformations_chain: &TransformationsChain,
    record: &ByteRecord,
    variables: &Variables,
    line_number: usize,
) -> String {
    let mut value: Option<String> = None;

    for transformation in transformations_chain.iter() {
        value = transformation.apply(value, record, variables, line_number);
    }

    match value {
        Some(content) => content,
        None => String::new(),
    }
}


fn transform(
    record: ByteRecord,
    transformer: &Transformer,
    variables: &Variables,
    line_number: usize,
) -> ByteRecord {
    let output: Vec<String> = transformer.columns.iter().map(
        |column| apply_transformations_chain(
            column,
            &record,
            &variables,
            line_number,
        )
    ).collect();

    ByteRecord::from(output)
}


pub fn process(options: Options) -> Result<(), String> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(io::stdin());

    let mut writer = Writer::from_writer(io::stdout());

    let headers = reader.headers().unwrap().clone();

    let maybe_transformer = create_transformer(
        &options.config,
        &headers,
        &options.variables,
    );

    if let Err(err) = maybe_transformer {
        return Err(err.error_description);
    }

    let transformer = maybe_transformer.unwrap();

    writer.write_record(&transformer.headers).unwrap();

    for (line_number, result) in reader.byte_records().enumerate() {
        let record = result.unwrap();

        writer.write_record(&transform(
            record,
            &transformer,
            &options.variables,
            line_number + 1,
        )).unwrap();
    }

    Ok(writer.flush().unwrap())
}
