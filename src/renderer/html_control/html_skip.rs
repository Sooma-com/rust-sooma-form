use abstract_form::renderer::FieldRenderer;

#[derive(Default)]
pub struct HtmlSkip {}
impl FieldRenderer for HtmlSkip {
    fn render(
        &self,
        _form: &abstract_form::Form,
        _form_renderer: &dyn abstract_form::renderer::FormRenderer,
        _fieldset: &abstract_form::FieldSet,
        _field: &std::sync::Arc<Box<dyn abstract_form::Field>>,
    ) -> String {
        "".to_string()
    }
}
