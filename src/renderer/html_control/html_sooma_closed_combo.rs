use abstract_form::renderer::FieldRenderer;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaClosedCombo {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub closed_choices: Vec<(String, String)>,
    pub attributes: HashMap<String, String>,
}
impl FieldRenderer for HtmlSoomaClosedCombo {
    fn render(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        _fieldset: &abstract_form::FieldSet,
        field: &std::sync::Arc<Box<dyn abstract_form::Field>>,
    ) -> String {
        let label = format!(
            r#"<label for="{}">{}</label>"#,
            encode_double_quoted_attribute(field.get_tag()),
            encode_safe(&field.get_label())
        );
        let integer_type_ids = [
            std::any::TypeId::of::<i8>(),
            std::any::TypeId::of::<i16>(),
            std::any::TypeId::of::<i32>(),
            std::any::TypeId::of::<i64>(),
            std::any::TypeId::of::<i128>(),
            std::any::TypeId::of::<isize>(),
            std::any::TypeId::of::<u8>(),
            std::any::TypeId::of::<u16>(),
            std::any::TypeId::of::<u32>(),
            std::any::TypeId::of::<u64>(),
            std::any::TypeId::of::<u128>(),
            std::any::TypeId::of::<usize>(),
        ];
        let float_type_ids = [std::any::TypeId::of::<f32>(), std::any::TypeId::of::<f64>()];
        let mut attributes = self.attributes.clone();
        attributes.entry("sergiosgc-enc".to_string()).or_insert(
            if integer_type_ids.contains(&field.inner_type_id()) {
                "integer"
            } else if float_type_ids.contains(&field.inner_type_id()) {
                "float"
            } else {
                "string"
            }
            .to_string(),
        );
        let input = format!(
            r#"<sooma-closed-combo name="{name}" value="{value}" {attributes}>{options}</sooma-closed-combo>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            attributes = attributes
                .iter()
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
            options = self
                .closed_choices
                .iter()
                .map(|(value, label)| format!(
                    r#"<option value="{}">{}</option>"#,
                    encode_double_quoted_attribute(value),
                    encode_safe(label)
                ))
                .join("")
        );
        let error_container = r#"<div class="error-message no-error"></div>"#.to_string();
        format!(
            r#"<div class="{}" data-name="{}">{label}{input}{error_container}</div>"#,
            ["sooma-form-control".to_string()]
                .iter()
                .chain(self.classes.iter())
                .map(|class| encode_double_quoted_attribute(class))
                .join(" "),
            encode_double_quoted_attribute(field.get_tag()),
        )
    }
}
