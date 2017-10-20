const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub fn get_version() -> String {
    VERSION.to_string()
}