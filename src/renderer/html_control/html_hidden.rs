use crate::renderer::html_control::sergiosgc_enc;
use abstract_form::renderer::FieldRenderer;
use html_escape::encode_double_quoted_attribute;

#[derive(Default)]
pub struct HtmlHidden {}
impl FieldRenderer for HtmlHidden {
    fn render(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        _fieldset: &abstract_form::FieldSet,
        field: &std::sync::Arc<Box<dyn abstract_form::Field>>,
    ) -> String {
        let input = format!(
            r#"<input type="hidden" name="{}" value="{}" sergiosgc-enc="{}" />"#,
            encode_double_quoted_attribute(field.get_tag()),
            encode_double_quoted_attribute(&field.get_value_as_string()),
            sergiosgc_enc(field)
        );
        input
    }
}
