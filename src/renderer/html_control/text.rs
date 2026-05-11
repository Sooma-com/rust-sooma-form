use abstract_form::renderer::FieldRenderer;
use html_escape::encode_double_quoted_attribute;

#[derive(Default)]
pub struct HtmlInput {}
impl FieldRenderer for HtmlInput {
    fn render(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        _fieldset: &abstract_form::FieldSet,
        field: &abstract_form::Field,
    ) -> String {
        format!(
            r#"<input type="text" name="{}" value="{}" />"#,
            encode_double_quoted_attribute(field.get_tag()),
            encode_double_quoted_attribute(&field.get_value_as_string())
        )
    }
}
