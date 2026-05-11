use crate::renderer::{HtmlFieldSetRenderer, html_control::text::HtmlInput};
use abstract_form::{
    Field,
    renderer::{FieldRenderer, FieldSetRenderer, FormRenderer},
};
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct HtmlFormRenderer {
    field_renderers: HashMap<String, Arc<Box<dyn FieldRenderer>>>,
    fieldset_renderers: HashMap<String, Arc<Box<dyn FieldSetRenderer>>>,
}
impl FormRenderer for HtmlFormRenderer {
    fn field_renderers(&self) -> &HashMap<String, Arc<Box<dyn FieldRenderer>>> {
        &self.field_renderers
    }

    fn field_renderers_mut(&mut self) -> &mut HashMap<String, Arc<Box<dyn FieldRenderer>>> {
        &mut self.field_renderers
    }

    fn fieldset_renderers(&self) -> &HashMap<String, Arc<Box<dyn FieldSetRenderer>>> {
        &self.fieldset_renderers
    }

    fn fieldset_renderers_mut(&mut self) -> &mut HashMap<String, Arc<Box<dyn FieldSetRenderer>>> {
        &mut self.fieldset_renderers
    }

    fn render_form_pre(&self, form: &abstract_form::Form) -> String {
        return "<form>".to_string();
    }

    fn render_form_post(&self, form: &abstract_form::Form) -> String {
        return "</form>".to_string();
    }

    fn get_default_field_renderer(&self, field: &Field) -> Arc<Box<dyn FieldRenderer>> {
        Arc::new(Box::new(HtmlInput::default()))
    }

    fn get_default_fieldset_renderer(
        &self,
        _fieldset: &abstract_form::FieldSet,
    ) -> Arc<Box<dyn FieldSetRenderer>> {
        Arc::new(Box::new(HtmlFieldSetRenderer::default()))
    }
}
