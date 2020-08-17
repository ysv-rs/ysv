use crate::compile::models::MaybeSomeTransformation;
use crate::transform::Transformation;


pub fn compile_date_with_multiple_formats(
    formats: &Vec<String>,
) -> MaybeSomeTransformation {
    Ok(Some(Transformation::DateMultiple {
        formats: formats.clone(),
    }))
}
