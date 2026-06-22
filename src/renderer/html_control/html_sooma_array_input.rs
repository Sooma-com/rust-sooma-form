use crate::renderer::html_control::sergiosgc_enc;
use abstract_form::renderer::FieldRenderer;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaArrayInput {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub r#type: Option<String>,
    pub attributes: HashMap<String, String>,
}
impl FieldRenderer for HtmlSoomaArrayInput {
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
        let inner_input = format!(
            r#"<input type="{}" />"#,
            encode_double_quoted_attribute(self.r#type.as_deref().unwrap_or("text"))
        );
        let mut attributes = self.attributes.clone();
        attributes
            .entry("sergiosgc-enc".to_string())
            .or_insert(sergiosgc_enc(field).to_string());
        let outer_input = format!(
            r#"<sooma-array name="{name}" value="{value}" {attributes}>{inner_input}</sooma-array>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            attributes = self
                .attributes
                .iter()
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
        );
        let error_container = r#"<div class="error-message no-error"></div>"#.to_string();
        // let error_container = if field.get_tag() == "parent" {
        //     format!(r#"<div class="error-message warning">This is a warning</div>"#)
        // } else if field.get_tag() == "last_access" {
        //     format!(r#"<div class="error-message error">This is an error</div>"#)
        // } else if field.get_tag() == "sync_guid" {
        //     format!(r#"<div class="error-message info">This is an info</div>"#)
        // } else {
        //     format!(r#"<div class="error-message no-error"></div>"#)
        // };
        format!(
            r#"<div class="{}" data-name="{}">{label}{outer_input}{error_container}</div>"#,
            ["sooma-form-control".to_string()]
                .iter()
                .chain(self.classes.iter())
                .map(|class| encode_double_quoted_attribute(class))
                .join(" "),
            encode_double_quoted_attribute(field.get_tag()),
        )
    }
}
