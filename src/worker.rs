use std::io;
use csv::{ByteRecord, Writer, ReaderBuilder, Reader};

use crate::compile::create_transformer;
use crate::transform::{Transformer, Transformation, CellValue};
use crate::options::Options;
use std::io::Stdout;

type TransformationsChain = Vec<Transformation>;
pub type MaybeTransformationsChain = Result<TransformationsChain, String>;


fn apply_transformations_chain(
    transformations_chain: &TransformationsChain,
    record: &ByteRecord,
    line_number: usize,
) -> String {
    let mut value: CellValue = CellValue::empty_string();

    for transformation in transformations_chain.iter() {
        value = transformation.apply(value, record, line_number);
    }

    value.to_string()
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


/// Read and process all the records from given CSV Reader object.
pub fn process_from_reader<T: io::Read>(
    mut reader: Reader<T>,
    options: &Options,
    writer: &mut Writer<Stdout>,
    start_line_number: usize,
) -> Result<usize, String> {
    let headers = reader.headers().unwrap().clone();

    let transformer = create_transformer(
        &options.config,
        &headers,
        &options.variables,
    )?;

    if start_line_number == 1 {
        writer.write_record(&transformer.headers).unwrap();
    }

    let mut current_line_number = start_line_number.clone();
    for (line_number, result) in reader.byte_records().enumerate() {
        let record = result.unwrap();

        current_line_number = start_line_number + line_number;

        writer.write_record(&transform(
            record,
            &transformer,
            current_line_number,
        )).unwrap();
    }

    writer.flush().unwrap();

    Ok(current_line_number + 1)
}


fn process_from_stdin(options: Options) -> Result<(), String> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(io::stdin());

    let mut writer = Writer::from_writer(io::stdout());

    process_from_reader(reader, &options, &mut writer, 1)?;

    Ok(())
}


fn process_from_file_list(options: &Options) -> Result<(), String> {
    let mut writer = Writer::from_writer(io::stdout());

    let mut line_number = 1;
    for file_path in options.input_files.as_ref().unwrap().iter() {
        let mut reader = ReaderBuilder::new()
            .flexible(true)
            .from_path(file_path).map_err(
                |err| err.to_string(),
            )?;

        line_number = process_from_reader(
            reader,
            &options,
            &mut writer,
            line_number,
        )?;
    }

    Ok(())
}


/// Do the whole job!
pub fn process(options: Options) -> Result<(), String> {
    match options.input_files {
        None => process_from_stdin(options),
        Some(_) => process_from_file_list(&options),
    }
}
