use crate::constants::DEFAULT_INSERT_PREFIX;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// If the plugin needs to import functions that aren't already imported, if will add this prefix
    #[serde(default = "default_alias")]
    pub func_alias_prefix: String,
}

fn default_alias() -> String {
    DEFAULT_INSERT_PREFIX.to_string()
}
