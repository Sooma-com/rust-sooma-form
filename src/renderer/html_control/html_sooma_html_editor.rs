use crate::renderer::html_control::{html_control_render, sergiosgc_enc};
use abstract_form::renderer::FieldRenderer;
use html_escape::encode_double_quoted_attribute;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaHtmlEditor {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
}
impl FieldRenderer for HtmlSoomaHtmlEditor {
    fn render(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        _fieldset: &abstract_form::FieldSet,
        field: &std::sync::Arc<Box<dyn abstract_form::Field>>,
    ) -> String {
        let mut attributes = self.attributes.clone();
        attributes
            .entry("sergiosgc-enc".to_string())
            .or_insert(sergiosgc_enc(field).to_string());
        let input = format!(
            r#"<sooma-html-editor name="{name}" value="{value}" {attributes}></sooma-html-editor>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            attributes = attributes
                .iter()
                .filter(|(key, _)| !key.starts_with('/'))
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
        );
        html_control_render(
            &input,
            ["sooma-form-control".to_string()]
                .iter()
                .chain(self.classes.iter())
                .cloned(),
            field,
            vec![],
            self.attributes.clone(),
        )
    }
}
