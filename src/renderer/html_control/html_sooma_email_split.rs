use abstract_form::renderer::FieldRenderer;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Default)]
pub struct HtmlSoomaEmailSplit {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub domain_field_name: String,
    pub options: Option<IndexMap<String, String>>,
}
impl FieldRenderer for HtmlSoomaEmailSplit {
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
        let options = self
            .options
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|(value, label)| {
                format!(
                    r#"<option value="{}">{}</option>"#,
                    encode_double_quoted_attribute(value),
                    encode_safe(label)
                )
            })
            .join("");
        let input = format!(
            r#"<sooma-email-split type="text" name="{name}" value="{value}" name-domain="{name_domain}" sergiosgc-enc="string">{options}</sooma-email-split>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            name_domain = encode_double_quoted_attribute(&self.domain_field_name),
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
