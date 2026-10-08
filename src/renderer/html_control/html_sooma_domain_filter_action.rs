use std::collections::HashMap;

use abstract_form::renderer::FieldRenderer;
use axum_gettext::__;
use html_escape::{encode_double_quoted_attribute, encode_text};
use itertools::Itertools;

use crate::renderer::html_control::{html_control_render, sergiosgc_enc};

pub struct HtmlSoomaDomainFilterFunction {
    pub label: String,
    pub function: String,
    pub param_labels: Vec<String>,
}

pub struct HtmlSoomaDomainFilterAction {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub functions: Vec<HtmlSoomaDomainFilterFunction>,
    pub conjunction_label: String,
    pub conjunction_label_and: String,
    pub conjunction_label_or: String,
    pub select_condition_label: String,
}
impl Default for HtmlSoomaDomainFilterAction {
    fn default() -> Self {
        Self {
            id: None,
            classes: vec![],
            attributes: HashMap::new(),
            functions: vec![
                HtmlSoomaDomainFilterFunction {
                    label: __!("Accept message and stop filtering", "en"),
                    function: "accept".to_string(),
                    param_labels: vec![],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Reject message and stop filtering", "en"),
                    function: "reject".to_string(),
                    param_labels: vec![],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Add recipient to BCC", "en"),
                    function: "bcc".to_string(),
                    param_labels: vec![__!("Recipient email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Change sender (From) into Reply To", "en"),
                    function: "from_to_replyto".to_string(),
                    param_labels: vec![__!("New sender email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Replace sender (From) address", "en"),
                    function: "replace_from".to_string(),
                    param_labels: vec![__!("New sender email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Remove recipient", "en"),
                    function: "remove_recipient".to_string(),
                    param_labels: vec![__!("Recipient email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Remove original message recipients", "en"),
                    function: "remove_all_recipients".to_string(),
                    param_labels: vec![],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Add recipient", "en"),
                    function: "add_recipient".to_string(),
                    param_labels: vec![__!("Recipient email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Add header", "en"),
                    function: "add_header".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Remove headers", "en"),
                    function: "remove_headers".to_string(),
                    param_labels: vec![__!("Headers", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Fix invalid message ID", "en"),
                    function: "fix_messageid".to_string(),
                    param_labels: vec![],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Whitelist message (skip spam checks)", "en"),
                    function: "whitelist".to_string(),
                    param_labels: vec![],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Import rules from domain", "en"),
                    function: "import".to_string(),
                    param_labels: vec![__!("Domain", "en")],
                },
            ],
            conjunction_label: __!("When", "en"),
            conjunction_label_and: __!("And", "en"),
            conjunction_label_or: __!("Or", "en"),
            select_condition_label: __!("Select condition to add...", "en"),
        }
    }
}

impl HtmlSoomaDomainFilterAction {
    pub fn localize(&mut self, lang: &str) {
        self.conjunction_label = __!(&self.conjunction_label, lang);
        self.conjunction_label_and = __!(&self.conjunction_label_and, lang);
        self.conjunction_label_or = __!(&self.conjunction_label_or, lang);
        self.select_condition_label = __!(&self.select_condition_label, lang);
        self.functions = self
            .functions
            .iter()
            .map(|function| HtmlSoomaDomainFilterFunction {
                label: __!(&function.label, lang),
                function: function.function.clone(),
                param_labels: function
                    .param_labels
                    .iter()
                    .map(|param_label| __!(param_label, lang))
                    .collect(),
            })
            .collect();
    }
}
impl FieldRenderer for HtmlSoomaDomainFilterAction {
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
            .entry("fixed_conjunction".to_string())
            .or_insert("and".to_string());
        attributes
            .entry("select_condition_label".to_string())
            .or_insert(self.select_condition_label.clone());

        let functions = self.functions
            .iter()
            .map(|f| {
                let params = f.param_labels
                    .iter()
                    .map(|label| {
                        format!(r#"<sooma-function-expression-function-param>{label}</sooma-function-expression-function-param>"#, 
                            label = encode_text(label))
                    })
                    .join("\n");
                format!(r#"<sooma-function-expression-function name="{name}" label="{label}">{params}</sooma-function-expression-function>"#,
                    name = encode_double_quoted_attribute(&f.function),
                    label = encode_double_quoted_attribute(&f.label),
                    params = params)
                })
            .join("\n");
        let input = format!(
            r#"<sooma-function-expression name="{name}" value="{value}" {attributes}>{functions}</sooma-function-expression>"#,
            name = encode_double_quoted_attribute(field.get_tag()),
            value = encode_double_quoted_attribute(&field.get_value_as_string()),
            attributes = attributes
                .iter()
                .filter(|(key, _)| !key.starts_with('/'))
                .map(|(key, value)| format!(
                    r#"{key}="{encoded_value}""#,
                    encoded_value = encode_double_quoted_attribute(value)
                ))
                .join(" "),
            functions = functions
        );
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
