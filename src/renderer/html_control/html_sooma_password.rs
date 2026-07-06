use crate::renderer::html_control::{html_control_render, sergiosgc_enc};
use abstract_form::renderer::FieldRenderer;
use html_escape::encode_double_quoted_attribute;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaPassword {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub json_enconding: Option<String>,
    pub attributes: HashMap<String, String>,
    pub min_length: Option<(String, u32, u32)>,
    pub uppercase_required: Option<(String, u32, bool)>,
    pub lowercase_required: Option<(String, u32, bool)>,
    pub uppercase_lowercase_required: Option<(String, u32, bool)>,
    pub digits_required: Option<(String, u32, bool)>,
    pub special_characters_required: Option<(String, u32, bool)>,
    pub double_entry: Option<String>,
    pub max_fail_score: Option<u32>,
    pub generate_password: Option<String>,
}
impl FieldRenderer for HtmlSoomaPassword {
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
        attributes
            .entry("max-fail-score".to_string())
            .or_insert("1".to_string());
        if let Some(generate_password) = self.generate_password.as_ref() {
            attributes
                .entry("generate-password".to_string())
                .or_insert(generate_password.to_string());
        }
        if let Some(double_entry) = self.double_entry.as_ref() {
            attributes
                .entry("double-entry".to_string())
                .or_insert(double_entry.to_string());
        }
        let mut requirements = String::new();
        if let Some((message, fail_score, min_length)) = self.min_length.as_ref() {
            requirements.push_str(&format!(r#"<sooma-password-check regex=".{{{min_length}}}" message="{message}" score="{fail_score}"></sooma-password-check>"#,
        message= encode_double_quoted_attribute(message)));
        }
        if let Some((message, fail_score, uppercase_required)) = self.uppercase_required.as_ref()
            && *uppercase_required
        {
            requirements.push_str(&format!(
                r#"<sooma-password-check regex="[A-Z]" message="{message}" score="{fail_score}"></sooma-password-check>"#,
                message= encode_double_quoted_attribute(message)
            ));
        }
        if let Some((message, fail_score, lowercase_required)) = self.lowercase_required.as_ref()
            && *lowercase_required
        {
            requirements.push_str(&format!(
                r#"<sooma-password-check regex="[a-z]" message="{message}" score="{fail_score}"></sooma-password-check>"#,
                message= encode_double_quoted_attribute(message)
            ));
        }
        if let Some((message, fail_score, uppercase_lowercase_required)) =
            self.uppercase_lowercase_required.as_ref()
            && *uppercase_lowercase_required
        {
            requirements.push_str(&format!(
                r#"<sooma-password-check regex="[a-z].*[A-Z]|[A-Z].*[a-z]" message="{message}" score="{fail_score}"></sooma-password-check>"#,
                message= encode_double_quoted_attribute(message)
            ));
        }
        if let Some((message, fail_score, digits_required)) = self.digits_required.as_ref()
            && *digits_required
        {
            requirements.push_str(&format!(
                r#"<sooma-password-check regex="[0-9]" message="{message}" score="{fail_score}"></sooma-password-check>"#,
                message= encode_double_quoted_attribute(message)
            ));
        }
        if let Some((message, fail_score, special_characters_required)) =
            self.special_characters_required.as_ref()
            && *special_characters_required
        {
            requirements.push_str(&format!(r#"<sooma-password-check regex="[^a-zA-Z0-9]" message="{message}" score="{fail_score}"></sooma-password-check>"#,
                message= encode_double_quoted_attribute(message)
            ));
        }
        let input = format!(
            r#"<sooma-password name="{name}" value="{value}" sergiosgc-enc="{sergiosgc_enc}" {attributes}>{requirements}</sooma-password>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            sergiosgc_enc = self
                .json_enconding
                .as_deref()
                .unwrap_or_else(|| sergiosgc_enc(field)),
            attributes = attributes
                .iter()
                .filter(|(key, _)| !key.starts_with('/'))
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
        );
        log::debug!("input: {}", input);
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
