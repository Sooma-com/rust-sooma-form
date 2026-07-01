pub enum Severity {
    Success,
    Info,
    Warning,
    Error,
}

pub struct ErrorMessage {
    pub severity: Severity,
    pub message: String,
}
