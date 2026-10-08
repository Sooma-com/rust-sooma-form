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

pub struct HtmlSoomaDomainFilterCondition {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub functions: Vec<HtmlSoomaDomainFilterFunction>,
    pub conjunction_label: String,
    pub conjunction_label_and: String,
    pub conjunction_label_or: String,
    pub select_condition_label: String,
}
impl Default for HtmlSoomaDomainFilterCondition {
    fn default() -> Self {
        Self {
            id: None,
            classes: vec![],
            attributes: HashMap::new(),
            functions: vec![
                HtmlSoomaDomainFilterFunction {
                    label: __!("Recipient exists", "en"),
                    function: "recipient_exists".to_string(),
                    param_labels: vec![__!("Recipient email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Recipient from a domain exists", "en"),
                    function: "recipient_domain_exists".to_string(),
                    param_labels: vec![__!("Recipient domain", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Communication occurs between authorized domains", "en"),
                    function: "between_authorized_domains".to_string(),
                    param_labels: vec![__!("Authorized domain list", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Communication occurs outside authorized domains", "en"),
                    function: "not between_authorized_domains".to_string(),
                    param_labels: vec![__!("Authorized domain list", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender is", "en"),
                    function: "sender_is".to_string(),
                    param_labels: vec![__!("Sender email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender domain is", "en"),
                    function: "sender_domain_is".to_string(),
                    param_labels: vec![__!("Sender domain", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender domain ends in", "en"),
                    function: "sender_domain_ends_with".to_string(),
                    param_labels: vec![__!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header exists", "en"),
                    function: "header_exists".to_string(),
                    param_labels: vec![__!("Header", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value matches regular expression", "en"),
                    function: "header_matches".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Regular expression", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value equals", "en"),
                    function: "header_equals".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header contains", "en"),
                    function: "header_contains".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Contained string", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header begins with", "en"),
                    function: "header_begins_with".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header ends with", "en"),
                    function: "header_ends_with".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Source SMTP server is", "en"),
                    function: "source_smtp_server".to_string(),
                    param_labels: vec![__!("Server IP address", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Recipient does not exist", "en"),
                    function: "not recipient_exists".to_string(),
                    param_labels: vec![__!("Recipient email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Recipient from a domain does not exist", "en"),
                    function: "not recipient_domain_exists".to_string(),
                    param_labels: vec![__!("Recipient domain", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender is not", "en"),
                    function: "not sender_is".to_string(),
                    param_labels: vec![__!("Sender email", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender domain is not", "en"),
                    function: "not sender_domain_is".to_string(),
                    param_labels: vec![__!("Sender domain", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Sender domain does not end in", "en"),
                    function: "not sender_domain_ends_with".to_string(),
                    param_labels: vec![__!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header does not exist", "en"),
                    function: "not header_exists".to_string(),
                    param_labels: vec![__!("Header", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value does not match regular expression", "en"),
                    function: "not header_matches".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Regular expression", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value does not equal", "en"),
                    function: "not header_equals".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value does not contain string", "en"),
                    function: "not header_contains".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Contained string", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value does not begin with string", "en"),
                    function: "not header_begins_with".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Header value does not end with string", "en"),
                    function: "not header_ends_with".to_string(),
                    param_labels: vec![__!("Header", "en"), __!("Value to test", "en")],
                },
                HtmlSoomaDomainFilterFunction {
                    label: __!("Source SMTP server is not", "en"),
                    function: "not source_smtp_server".to_string(),
                    param_labels: vec![__!("Server IP address", "en")],
                },
            ],
            conjunction_label: __!("When", "en"),
            conjunction_label_and: __!("And", "en"),
            conjunction_label_or: __!("Or", "en"),
            select_condition_label: __!("Select condition to add...", "en"),
        }
    }
}

impl HtmlSoomaDomainFilterCondition {
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
impl FieldRenderer for HtmlSoomaDomainFilterCondition {
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
            .entry("conjunction_label".to_string())
            .or_insert(self.conjunction_label.clone());
        attributes
            .entry("conjunction_label_and".to_string())
            .or_insert(self.conjunction_label_and.clone());
        attributes
            .entry("conjunction_label_or".to_string())
            .or_insert(self.conjunction_label_or.clone());
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
