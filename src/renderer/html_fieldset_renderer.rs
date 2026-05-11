use abstract_form::renderer::FieldSetRenderer;
use html_escape::encode_safe;

#[derive(Default)]
pub struct HtmlFieldSetRenderer {}
impl FieldSetRenderer for HtmlFieldSetRenderer {
    fn render_fieldset_pre(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        fieldset: &abstract_form::FieldSet,
    ) -> String {
        if fieldset.label.is_empty() {
            return "".to_string();
        }
        format!(
            r#"<fieldset><legend>{}</legend>"#,
            encode_safe(&fieldset.label)
        )
    }

    fn render_fieldset_post(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        fieldset: &abstract_form::FieldSet,
    ) -> String {
        if fieldset.label.is_empty() {
            return "".to_string();
        }
        "</fieldset>".to_string()
    }
}
