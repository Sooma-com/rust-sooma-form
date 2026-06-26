use crate::renderer::html_control::sergiosgc_enc;
use abstract_form::{
    field::get_validations_by_type, renderer::FieldRenderer, validation::ClosedSingleChoice,
};
use html_escape::{encode_double_quoted_attribute, encode_safe};
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct HtmlSoomaSingleClosedChoice {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
}
impl FieldRenderer for HtmlSoomaSingleClosedChoice {
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
        let input = {
            let options: IndexMap<String, String> = match (
                get_validations_by_type::<ClosedSingleChoice<String>>(field).next(),
                get_validations_by_type::<ClosedSingleChoice<bool>>(field).next(),
            ) {
                (Some(validation), _) => validation
                    .as_any()
                    .downcast_ref::<ClosedSingleChoice<String>>()
                    .unwrap()
                    .options
                    .iter()
                    .map(|(value, label)| (value.to_string(), label.clone()))
                    .collect(),
                (_, Some(validation)) => validation
                    .as_any()
                    .downcast_ref::<ClosedSingleChoice<bool>>()
                    .unwrap()
                    .options
                    .iter()
                    .map(|(value, label)| (value.to_string(), label.clone()))
                    .collect(),
                (None, None) => {
                    if field.inner_type_id() == std::any::TypeId::of::<bool>() {
                        [
                            ("true".to_string(), "True".to_string()),
                            ("false".to_string(), "False".to_string()),
                        ]
                        .into_iter()
                        .collect()
                    } else {
                        IndexMap::new()
                    }
                }
            };
            let mut attributes = self.attributes.clone();
            attributes
                .entry("sergiosgc-enc".to_string())
                .or_insert(sergiosgc_enc(field).to_string());
            let open_tag = format!(
                r#"<sooma-single-closed-choice name="{name}" value="{value}" {attributes}>"#,
                name = encode_double_quoted_attribute(field.get_tag()),
                value = encode_double_quoted_attribute(&field.get_value_as_string()),
                attributes = attributes
                    .iter()
                    .map(|(key, value)| format!(
                        r#"{key}="{encoded_value}""#,
                        encoded_value = encode_double_quoted_attribute(value)
                    ))
                    .join(" "),
            );
            let mut option_tags = Vec::<String>::new();
            for (key, value) in options {
                if field.get_value_as_string() == key {
                    option_tags.push(format!(
                        r#"<option value="{}" selected>{}</option>"#,
                        encode_double_quoted_attribute(&key),
                        encode_safe(&value)
                    ));
                } else {
                    option_tags.push(format!(
                        r#"<option value="{}">{}</option>"#,
                        encode_double_quoted_attribute(&key),
                        encode_safe(&value)
                    ));
                }
            }
            let close_tag = r#"</sooma-single-closed-choice>"#.to_string();
            [open_tag]
                .iter()
                .chain(option_tags.iter())
                .chain([close_tag].iter())
                .map(|tag| tag.to_string())
                .join("")
        };
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
