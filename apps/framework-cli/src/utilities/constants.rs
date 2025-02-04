use lazy_static::lazy_static;
use std::collections::HashMap;
use uuid::Uuid;

pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const PACKAGE_JSON: &str = "package.json";
pub const SETUP_PY: &str = "setup.py";
pub const OLD_PROJECT_CONFIG_FILE: &str = "project.toml";
pub const PROJECT_CONFIG_FILE: &str = "moose.config.toml";
pub const PROJECT_NAME_ALLOW_PATTERN: &str = r"^[a-zA-Z0-9_-]+$";

pub const CLI_CONFIG_FILE: &str = "config.toml";
pub const CLI_USER_DIRECTORY: &str = ".moose";
pub const CLI_PROJECT_INTERNAL_DIR: &str = ".moose";
pub const CLI_INTERNAL_VERSIONS_DIR: &str = "versions";
pub const CLI_DEV_REDPANDA_VOLUME_DIR: &str = "redpanda";
pub const CLI_DEV_CLICKHOUSE_VOLUME_DIR_LOGS: &str = "clickhouse/logs";
pub const CLI_DEV_CLICKHOUSE_VOLUME_DIR_DATA: &str = "clickhouse/data";
pub const CLI_DEV_CLICKHOUSE_VOLUME_DIR_CONFIG_SCRIPTS: &str = "clickhouse/configs/scripts";
pub const CLI_DEV_CLICKHOUSE_VOLUME_DIR_CONFIG_USERS: &str = "clickhouse/configs/users";

pub const SCHEMAS_DIR: &str = "datamodels";
pub const FUNCTIONS_DIR: &str = "functions";
pub const AGGREGATIONS_DIR: &str = "aggregations";
pub const BLOCKS_DIR: &str = "blocks";
pub const CONSUMPTION_DIR: &str = "apis";
pub const VSCODE_DIR: &str = ".vscode";
pub const SAMPLE_STREAMING_FUNCTION_SOURCE: &str = "UserActivity";
pub const SAMPLE_STREAMING_FUNCTION_DEST: &str = "ParsedActivity";

pub const CLICKHOUSE_CONTAINER_NAME: &str = "clickhousedb-1";
pub const REDPANDA_CONTAINER_NAME: &str = "redpanda-1";

pub const REDPANDA_HOSTS: [&str; 2] = ["redpanda", "localhost"];

pub const APP_DIR: &str = "app";
pub const APP_DIR_LAYOUT: [&str; 4] = [
    SCHEMAS_DIR,
    FUNCTIONS_DIR,
    AGGREGATIONS_DIR,
    CONSUMPTION_DIR,
];

pub const GITIGNORE: &str = ".gitignore";

// These two constants are for the old convention of nested directories
// we will not be renaming them
pub const TS_FLOW_FILE: &str = "flow.ts";
pub const PY_FLOW_FILE: &str = "flow.py";

pub const TS_AGGREGATIONS_FILE: &str = "DailyActiveUsers.ts";
pub const PY_AGGREGATIONS_FILE: &str = "DailyActiveUsers.py";
pub const API_FILE: &str = "dailyActiveUsers.ts";

pub const VSCODE_EXT_FILE: &str = "extensions.json";
pub const VSCODE_SETTINGS_FILE: &str = "settings.json";

pub const CTX_SESSION_ID: &str = "session_id";

pub const PYTHON_FILE_EXTENSION: &str = "py";
pub const TYPESCRIPT_FILE_EXTENSION: &str = "ts";
pub const SQL_FILE_EXTENSION: &str = "sql";

pub const PYTHON_INIT_FILE: &str = "__init__.py";

lazy_static! {
    pub static ref CONTEXT: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(CTX_SESSION_ID.to_string(), Uuid::new_v4().to_string());
        map
    };
}

pub const README_PREFIX: &str = r#"
This is a [MooseJs](https://www.moosejs.com/) project bootstrapped with the 
[`Moose CLI`](https://github.com/514-labs/moose/tree/main/apps/framework-cli).

"#;
