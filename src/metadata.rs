/* -------------------------------------------------------------------------- */
/* GRC Metadata */
/* -------------------------------------------------------------------------- */

pub const VERSION: &str = "1.3.0";
pub const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
pub const NAME: &str = "GRC";
pub const DESCRIPTION: &str = r#"
I'm here to help you make it more standardized and convenient to use Git.
"#;

/* -------------------------------------------------------------------------- */
/* Constant */
/* -------------------------------------------------------------------------- */

pub const SEPARATOR_SYMBOL: &str = ":";
pub const SPACE: &str = " ";

pub const GIT_AUTHOR_NAME: &str = "GIT_AUTHOR_NAME";
pub const GIT_AUTHOR_EMAIL: &str = "GIT_AUTHOR_EMAIL";
pub const GIT_COMMITTER_NAME: &str = "GIT_COMMITTER_NAME";
pub const GIT_COMMITTER_EMAIL: &str = "GIT_COMMITTER_EMAIL";

pub const GRC_CONFIG_FILE_NAME: &str = "grc.toml";
pub const GLOBAL_CONFIG_PATH: &str = ".config/grc/grc.toml";

/* -------------------------------------------------------------------------- */
/* Base Commit Types */
/* -------------------------------------------------------------------------- */

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

pub const BASE_COMMIT_TYPE_EMOJI: &[(&str, &str)] = &[
	("test", "🧪"),
	("feat", "🎉"),
	("fix", "🐞"),
	("chore", "📦"),
	("docs", "📝"),
	("refactor", "✂ "),
	("style", "🎨"),
	("perf", "⚡"),
	("ci", "🚀"),
];

/* -------------------------------------------------------------------------- */
/* GRC Commit Mode */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
	Add,
	AddAll,
	Commit,
	Version,
}

/* --------------------------------------------------------------------------- */
/* CLI Add Command */
/* --------------------------------------------------------------------------- */

pub const ADD_COMMAND: &str = "add";
pub const ADD_PARAMS: &str = "filename";
pub const ADD_COMMAND_SHORT: char = 'a';
pub const ADD_COMMAND_HELP: &str = "Help you add files before commit. If the parameter is `.`, Then GRC will help you add everything.";
pub const ADD_COMMAND_NO_FILE: &str = "The add command requires parameters.";

/* -------------------------------------------------------------------------- */
/* CLI Designate Config File Command */
/* -------------------------------------------------------------------------- */

pub const DESIGNATE_CONFIG_COMMAND: &str = "config";
pub const DESIGNATE_CONFIG_PARAMS: &str = "configfile";
pub const DESIGNATE_CONFIG_COMMAND_SHORT: char = 'c';
pub const DESIGNATE_CONFIG_COMMAND_HELP: &str =
	"Manually specify a configuration file for the GRC.";

/* -------------------------------------------------------------------------- */
/* CLI Enable EMOJI */
/* -------------------------------------------------------------------------- */

pub const EMOJI_COMMAND: &str = "emoji";
pub const EMOJI_COMMAND_HELP: &str = "Make your submission record look beautiful.";

/* -------------------------------------------------------------------------- */
/* CLI VERSION INFO */
/* -------------------------------------------------------------------------- */

pub const VERSION_COMMAND: &str = "version";
pub const VERSION_COMMAND_SHORT: char = 'v';
pub const VERSION_COMMAND_HELP: &str = "Show the version.";

/* -------------------------------------------------------------------------- */
/* GRC ERROR */
/* -------------------------------------------------------------------------- */

pub const TYPE_PARSE_FAILED: &str = "Configuration File Parse Failed: ** type ** Is not correct.";
pub const OVERWRITE_PARSE_FAILED: &str =
	"Configuration File Parse Failed: ** overwrite_emoji ** Is not correct.";
