use std::collections::HashMap;

pub mod html_hidden;
pub mod html_input;
pub mod html_select;
pub mod html_skip;
pub mod html_sooma_email_split;
pub mod html_sooma_multiple_closed_choice;
pub mod html_sooma_single_closed_choice;
pub mod layout;
pub fn sergiosgc_enc(field: &std::sync::Arc<Box<dyn abstract_form::Field>>) -> &str {
    let type_id_map = HashMap::from([
        (std::any::TypeId::of::<String>(), "string"),
        (std::any::TypeId::of::<Option<String>>(), "optional_string"),
        (std::any::TypeId::of::<bool>(), "boolean"),
        (std::any::TypeId::of::<i8>(), "integer"),
        (std::any::TypeId::of::<i16>(), "integer"),
        (std::any::TypeId::of::<i32>(), "integer"),
        (std::any::TypeId::of::<i64>(), "integer"),
        (std::any::TypeId::of::<i128>(), "integer"),
        (std::any::TypeId::of::<isize>(), "integer"),
        (std::any::TypeId::of::<u8>(), "integer"),
        (std::any::TypeId::of::<u16>(), "integer"),
        (std::any::TypeId::of::<u32>(), "integer"),
        (std::any::TypeId::of::<u64>(), "integer"),
        (std::any::TypeId::of::<u128>(), "integer"),
        (std::any::TypeId::of::<usize>(), "integer"),
        (std::any::TypeId::of::<f32>(), "float"),
        (std::any::TypeId::of::<f64>(), "float"),
        (std::any::TypeId::of::<Vec<String>>(), "string[]"),
        (
            std::any::TypeId::of::<Vec<Option<String>>>(),
            "optional_string[]",
        ),
        (std::any::TypeId::of::<Vec<bool>>(), "boolean[]"),
        (std::any::TypeId::of::<Vec<i8>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<i16>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<i32>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<i64>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<i128>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<isize>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<u8>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<u16>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<u32>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<u64>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<u128>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<usize>>(), "integer[]"),
        (std::any::TypeId::of::<Vec<f32>>(), "float[]"),
        (std::any::TypeId::of::<Vec<f64>>(), "float[]"),
    ]);
    match type_id_map.get(&field.inner_type_id()) {
        Some(type_name) => type_name,
        None => "string",
    }
}
