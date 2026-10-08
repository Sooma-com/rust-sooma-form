use axum_gettext::transpile_mo;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    transpile_mo(
        &format!(
            "{}/private/locale",
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set")
        ),
        &format!(
            "{}/src/localization/mod.rs",
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set")
        ),
    )
}

fn main() {
    run().expect("Failed to transpile MO files");
}
