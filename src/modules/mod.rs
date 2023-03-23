use serde::Deserialize;

pub mod battery;

#[derive(Debug, Deserialize)]
pub struct ModuleConfig {
    pub brackets: Option<bool>,
    pub label: Option<String>,
    pub show_label: Option<bool>,
}

#[derive(Debug)]
struct ModuleParams {
    brackets: bool,
    label: String,
    show_label: bool,
}
