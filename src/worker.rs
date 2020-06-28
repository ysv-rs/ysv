use std::io;
use csv::{ByteRecord, Writer, ReaderBuilder};

use crate::config::create_transformer;
use crate::transformer::{Transformer, Transformation};
use crate::options::Options;
use crate::printable_error::ConfigParseError;


type TransformationsChain = Vec<Transformation>;
pub type MaybeTransformationsChain = Result<TransformationsChain, ConfigParseError>;


fn apply_transformations_chain(
    transformations_chain: &TransformationsChain,
    record: &ByteRecord,
    line_number: usize,
) -> String {
    let mut value: Option<String> = None;

    for transformation in transformations_chain.iter() {
        value = transformation.apply(value, record, line_number);
    }

    value.unwrap_or("".to_string())
}


fn transform(
    record: ByteRecord,
    transformer: &Transformer,
    line_number: usize,
) -> ByteRecord {
    let output: Vec<String> = transformer.columns.iter().map(
        |column| apply_transformations_chain(
            column,
            &record,
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

    if let Err(err) = maybe_transformer {   // FIXME this is too hard
        return Err(err.error_description);  // FIXME and this too
    }

    let transformer = maybe_transformer.unwrap();

    writer.write_record(&transformer.headers).unwrap();

    for (line_number, result) in reader.byte_records().enumerate() {
        let record = result.unwrap();

        writer.write_record(&transform(
            record,
            &transformer,
            line_number + 1,
        )).unwrap();
    }

    Ok(writer.flush().unwrap())
}
