use abstract_form::renderer::FieldRenderer;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaImageCrop {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub attributes: HashMap<String, String>,
}
impl FieldRenderer for HtmlSoomaImageCrop {
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
        let mut attributes = self.attributes.clone();
        attributes
            .entry("sergiosgc-enc".to_string())
            .or_insert("string".to_string());
        let input = format!(
            r#"<sooma-image-crop name="{name}" value="{value}" {attributes} width="{width}" height="{height}"></sooma-image-crop>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            attributes = attributes
                .iter()
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
            width = self.width,
            height = self.height,
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
