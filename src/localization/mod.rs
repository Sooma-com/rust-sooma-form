use std::collections::HashMap;
#[allow(clippy::type_complexity)]
pub static TRANSLATIONS: std::sync::LazyLock<HashMap<String, HashMap<(String, Option<String>), String>>> = std::sync::LazyLock::new(|| 
    [
    ].into_iter().collect::<HashMap<String, HashMap<(String, Option<String>), String>>>()
);