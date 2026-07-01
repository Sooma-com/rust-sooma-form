use crate::renderer::html_control::error_message::{ErrorMessage, Severity};
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;
use std::collections::HashMap;

pub mod error_message;
pub mod html_hidden;
pub mod html_input;
pub mod html_select;
pub mod html_skip;
pub mod html_sooma_array_input;
pub mod html_sooma_closed_combo;
pub mod html_sooma_email_split;
pub mod html_sooma_image_crop;
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
pub fn html_control_render(
    inner_control: &str,
    classes: impl IntoIterator<Item = String>,
    field: &std::sync::Arc<Box<dyn abstract_form::Field>>,
    error_messages: impl IntoIterator<Item = ErrorMessage>,
    attributes: impl IntoIterator<Item = (String, String)>,
) -> String {
    let mut error_messages = error_messages.into_iter().peekable();
    let error_div = if error_messages.peek().is_none() {
        r#"<div class="error-message no-error"></div>"#.to_string()
    } else {
        format!(
            r#"<div class="error-message">{error_messages}</div>"#,
            error_messages = error_messages
                .map(|message| format!(
                    r#"<span class="{class}">{message}</span>"#,
                    class = match message.severity {
                        Severity::Success => "success",
                        Severity::Info => "info",
                        Severity::Warning => "warning",
                        Severity::Error => "error",
                    },
                    message = &message.message
                ))
                .join("")
        )
    };
    format!(r#"<div class="{classes}" data-name="{field_tag}" {attributes}><label for="{field_tag}">{field_label}</label>{inner_control}{error_div}</div>"#,
        classes = classes.into_iter().map(|class| encode_double_quoted_attribute(&class).to_string()).join(" "),
        field_tag = encode_double_quoted_attribute(field.get_tag()),
        attributes = attributes
            .into_iter()
            .filter( |(key, _)| key.starts_with('/') )
            .map( |(key, value)| (key.trim_start_matches('/').to_string(), value) )
            .map(|(key, value)| format!(r#"{key}="{value}""#, 
                key = key,
                value = encode_double_quoted_attribute(&value),
            ))
            .join(" "),
        field_label = encode_safe(&field.get_label()),
        inner_control = inner_control,
        error_div = error_div,
    ).to_string()
}
