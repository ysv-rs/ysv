use crate::transform::Transformation;
use crate::compile::models::{InputColumnIndexByName, MaybeSomeTransformation};


/// Compiles the specified input column name to a Transformation with the index of the said column.
pub fn compile_singular_input(
    input_column_name: &String,
    input_column_index_by_name: &InputColumnIndexByName,
) -> MaybeSomeTransformation {
    let input_column_index = input_column_index_by_name.get(
        input_column_name,
    );

    if input_column_index.is_none() {
        // FIXME this should not be here
        eprintln!("Warning: input column {} not found.", input_column_name);
    }

    Ok(input_column_index.map(
        |index| Transformation::Input(index.clone()),
    ))
}


/// Provided a list of several input column names, find the first column which actually
/// exists in the input stream, and use it. Useful to coerce multiple schemas to one.
pub fn compile_multiple_input(
    input_column_names: &Vec<String>,
    input_column_index_by_name: &InputColumnIndexByName,
) -> MaybeSomeTransformation {
    let maybe_input_column_index: Option<&usize> = input_column_names.iter().map(
        |column_name| input_column_index_by_name.get(column_name),
    ).flatten().next();

    Ok(maybe_input_column_index.map(
        |index| Transformation::Input(index.clone()),
    ))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_compile_multiple_input_first() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("date".to_string(), 2);
        indices.insert("Date".to_string(), 1);

        let transformation = compile_multiple_input(
            &names,
            &indices,
        ).unwrap().unwrap();

        // FIXME
        // assert_eq!(
        //     transformation,
        //     Transformation::Input(1),
        // )
    }


    fn test_compile_multiple_input_second() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
            "Transaction Date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("Transaction Date".to_string(), 5);
        indices.insert("Event Date".to_string(), 1);

        let transformation = compile_multiple_input(
            &names,
            &indices,
        ).unwrap().unwrap();

        // assert_eq!(
        //     transformation,
        //     Transformation::Input(5),
        // )
    }

    fn test_compile_multiple_input_empty() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
            "Transaction Date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("Happening Date".to_string(), 5);
        indices.insert("Event Date".to_string(), 1);

        let maybe_some_transformation = compile_multiple_input(
            &names,
            &indices,
        );

        assert!(maybe_some_transformation.unwrap().is_none());
    }
}
