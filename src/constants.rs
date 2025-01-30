// Nodejs packages that we want need in our shims
pub const PATH_PKG: &str = "path";
pub const NODE_PATH_PKG: &str = "node:path";
pub const URL_PKG: &str = "url";
pub const NODE_URL_PKG: &str = "node:url";
pub const DIRNAME_FUNC: &str = "dirname";
pub const URL_TO_FILE_PATH_FUNC: &str = "fileURLToPath";

// Comonjs Globals that we want to shim forward
pub const DIRNAME_TOKEN: &str = "__dirname";
pub const FILENAME_TOKEN: &str = "__filename";

// Defaults
pub const DEFAULT_INSERT_PREFIX: &str = "__swc_shim_";

// Plugin Name
pub const PLUGIN_NAME: &str = "node-globals-inject";
