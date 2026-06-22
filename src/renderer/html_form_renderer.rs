use crate::renderer::{
    HtmlFieldSetRenderer,
    html_control::{html_input::HtmlInput, html_select::HtmlSelect},
};
use abstract_form::{
    Field,
    field::get_validations_by_type,
    renderer::{FieldRenderer, FieldSetRenderer, FormRenderer},
    validation::ClosedSingleChoice,
};
use html_escape::{encode_double_quoted_attribute, encode_safe};
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct HtmlFormRenderer {
    pub field_renderers: HashMap<String, Arc<Box<dyn FieldRenderer>>>,
    pub fieldset_renderers: HashMap<String, Arc<Box<dyn FieldSetRenderer>>>,
    pub classes: Vec<String>,
    pub id: Option<String>,
    pub action: Option<String>,
    pub attributes: HashMap<String, String>,
    pub submit: Vec<(String, String)>,
}
impl HtmlFormRenderer {
    pub fn add_class(&mut self, class: &str) {
        self.classes.push(class.to_string());
    }
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    pub fn add_submit(&mut self, label: &str, value: &str) {
        self.submit.push((label.to_string(), value.to_string()));
    }
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
    fn render_form_pre(&self, _form: &abstract_form::Form) -> String {
        let mut attributes = self.attributes.clone();
        if let Some(action) = self.action.as_ref() {
            attributes.insert("action".to_string(), action.clone());
        }
        if let Some(id) = self.id.as_ref() {
            attributes.insert("id".to_string(), id.clone());
        }
        attributes.insert(
            "class".to_string(),
            ["sooma-form".to_string()]
                .iter()
                .chain(self.classes.iter())
                .map(|class| encode_double_quoted_attribute(class))
                .join(" "),
        );
        format!(
            r#"<form {}>"#,
            attributes
                .iter()
                .map(|(key, value)| format!(
                    r#"{}="{}""#,
                    key,
                    encode_double_quoted_attribute(value)
                ))
                .join(" ")
        )
    }

    fn render_form_post(&self, _form: &abstract_form::Form) -> String {
        format!(
            r#"
<span class="submit-buttons">{}</span></form>
            "#,
            self.submit
            .iter()
            .map(|(action, label)| format!(r#"<button type="submit" class="submit submit-{} "name="submit" value="{}">{}</button>"#, 
                encode_double_quoted_attribute(action),
                encode_double_quoted_attribute(action),
                encode_safe(label)))
            .join(" ")
        )
    }

    fn get_default_field_renderer(
        &self,
        field: &std::sync::Arc<Box<dyn Field>>,
    ) -> Arc<Box<dyn FieldRenderer>> {
        if field.inner_type_id() == std::any::TypeId::of::<bool>()
            || field.inner_type_id() == std::any::TypeId::of::<Option<bool>>()
        {
            return Arc::new(Box::new(HtmlSelect::default()));
        }
        if get_validations_by_type::<ClosedSingleChoice<String>>(field).count() > 0
            || get_validations_by_type::<ClosedSingleChoice<bool>>(field).count() > 0
        {
            return Arc::new(Box::new(HtmlSelect::default()));
        }
        Arc::new(Box::new(HtmlInput::default()))
    }

    fn get_default_fieldset_renderer(
        &self,
        _fieldset: &abstract_form::FieldSet,
    ) -> Arc<Box<dyn FieldSetRenderer>> {
        Arc::new(Box::new(HtmlFieldSetRenderer::default()))
    }
}
