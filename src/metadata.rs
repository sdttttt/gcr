pub const VERSION: &str = "0.9.0-rc1";
pub const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
pub const NAME: &str = "GRC";
pub const DESCRIPTION: &str = r#"
	I'm here to help you make it more standardized and convenient to use Git.
	"#;

pub const SEPARATOR_SYMBOL: &str = ":";

pub const SPACE: &str = " ";

pub const GIT_AUTHOR_NAME: &str = "GIT_AUTHOR_NAME";
pub const GIT_AUTHOR_EMAIL: &str = "GIT_AUTHOR_EMAIL";

pub const GLOBAL_CONFIG_PATH: &str = ".config/grc/grc.toml";

pub const BASE_COMMIT_TYPE_DESCRIPTION: &[(&str, &str)] = &[
    ("test", "Adding missing tests."),
    ("feat", "A new feature."),
    ("fix", "A bug fix."),
    ("chore", "Build process or auxiliary tool changes."),
    ("docs", "Documentation only changes."),
    ("refactor", "A code change that neither fixes a bug or adds a feature."),
    ("style", "Markup, white-space, formatting, missing semi-colons..."),
    ("perf", "A code change that improves performance."),
    ("ci", "CI related changes."),
];

// GRC four commit modes.
pub enum Mode {
    Auto,
    Add,
    AddAll,
    Commit,
    Push,
}

pub const ADD_COMMAND: &str = "add";
pub const ADD_PARAMS: &str = "add_file_name";
pub const ADD_COMMAND_SHORT: &str = "a";
pub const ADD_COMMAND_HELP: &str = "Help you add files before commit. If the parameter is `.`, Then GRC will help you add everything.";
// Not found file execption information.
pub const ADD_COMMAND_NO_FILE: &str = "The add command requires parameters.";

pub const PUSH_COMMAND: &str = "push";
pub const PUSH_PARAMS: &str = "push_file_name";
pub const PUSH_COMMAND_SHORT: &str = "p";

pub const PUSH_COMMAND_HELP: &str = "...";
pub const PUSH_COMMAND_NO_FILE: &str = "The 'push' command requires parameters.";

pub const GRC_CONFIG_FILE_NAME: &str = "grc.toml";
