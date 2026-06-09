use abstract_form::renderer::FieldRenderer;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;

#[derive(Default)]
pub struct HtmlSoomaImageCrop {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub json_enconding: Option<String>,
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
        let input = format!(
            r#"<sooma-image-crop name="{}" value="{}" sergiosgc-enc="{}" width="{}" height="{}"></sooma-image-crop>"#,
            encode_double_quoted_attribute(field.get_tag()),
            encode_double_quoted_attribute(&field.get_value_as_string()),
            self.json_enconding.as_deref().unwrap_or("string"),
            self.width,
            self.height,
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
