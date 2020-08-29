use std::{io, thread};
use csv::{ByteRecord, Writer, ReaderBuilder, Reader};

use crate::compile::create_transformer;
use crate::transform::{Transformer, Transformation, CellValue, ApplyResult};
use crate::options::Options;
use std::io::Stdout;
use std::sync::mpsc;
use crate::writer::writer_thread;

type TransformationsChain = Vec<Transformation>;
pub type MaybeTransformationsChain = Result<TransformationsChain, String>;

/// Apply the given chain of transformations to the value given.
/// The return value is ready for printing, hence it is a String.
fn apply_transformations_chain(
    transformations_chain: &TransformationsChain,
    record: &ByteRecord,
    line_number: usize,
) -> String {
    // TODO it would be useful for performance to stop fold()-ing when the value is Err().
    let apply_result: ApplyResult = transformations_chain.iter().fold(
        Ok(CellValue::empty_string()),
        |result, transformation| result.and_then(
            |cell_value| transformation.apply(
                cell_value,
                record,
                line_number,
            )
        )
    );

    if let Ok(cell_value) = apply_result {
        cell_value.to_string()
    } else {
        eprintln!("{}", apply_result.err().unwrap());
        String::new()
    }
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
    start_line_number: usize,
) -> Result<usize, String> {
    let headers = reader.headers().unwrap().clone();

    let transformer = create_transformer(
        &options.config,
        &headers,
        &options.variables,
    )?;

    let (tx, rx) = mpsc::channel();

    if start_line_number == 1 {
        tx.send(transformer.headers.as_byte_record().clone()).unwrap();
    }

    let writer_handle = thread::spawn(move || writer_thread(rx));

    let mut current_line_number = start_line_number.clone();
    for (line_number, result) in reader.byte_records().enumerate() {
        let record = result.unwrap();

        current_line_number = start_line_number + line_number;

        tx.send(transform(
            record,
            &transformer,
            current_line_number,
        )).unwrap();
    }

    // We must close the channel to indicate we are not going to send anything else
    drop(tx);

    writer_handle.join().unwrap();

    Ok(current_line_number + 1)
}


/// Read CSV data from standard input.
fn process_from_stdin(options: Options) -> Result<(), String> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(io::stdin());

    process_from_reader(reader, &options, 1)?;

    Ok(())
}


/// Read CSV data from a set of files.
fn process_from_file_list(options: &Options) -> Result<(), String> {
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
