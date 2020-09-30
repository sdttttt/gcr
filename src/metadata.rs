pub const VERSION: &str = "0.8.0";
pub const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
pub const NAME: &str = "GRC";
pub const DESCRIPTION: &str =
    "I'm here to help you make it more standardized and convenient to use Git.";

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
pub const ADD_COMMAND_NO_FILE: &str = "The add command requires parameters.";

pub const PUSH_COMMAND: &str = "push";
pub const PUSH_PARAMS: &str = "push_file_name";
pub const PUSH_COMMAND_SHORT: &str = "p";
// TODO: Fill in the `push` command help
pub const PUSH_COMMAND_HELP: &str = "...";
pub const PUSH_COMMAND_NO_FILE: &str = "The 'push' command requires parameters.";
