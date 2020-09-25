use dialoguer::{theme::ColorfulTheme, Input, Select};

const COMMIT_TYPES_DESCRIPTION: &[&str] = &[
    "test:       Adding missing tests.",
    "feat:       A new feature.",
    "fix:        A bug fix.",
    "chore:      Build process or auxiliary tool changes.",
    "docs:       Documentation only changes.",
    "refactor:   A code change that neither fixes a bug or adds a feature.",
    "style:      Markup, white-space, formatting, missing semi-colons...",
    "perf:       A code change that improves performance.",
    "ci:         CI related changes.",
    "Custom your type.",
];

const COMMIT_TYPES: &[&str] = &[
    "test", "feat", "fix", "chore", "docs", "refactor", "style", "perf", "ci",
];

pub struct Messager {
    typ: String,
    scope: String,
    content: String,
}

impl Messager {
    pub fn new() -> Self {
        let typ = ask_type();
        let scope = ask_scope();
        let content = ask_content();
        Self {
            typ,
            scope,
            content,
        }
    }

    pub fn build(&self) -> String {
        if self.scope.len() == 0 || self.scope.trim() == "" {
            format!("{}: {}", self.typ, self.content)
        } else {
            format!("{}({}): {}", self.typ, self.scope, self.content)
        }
    }
}

fn ask_type() -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(COMMIT_TYPES_DESCRIPTION)
        .default(0)
        .interact()
        .unwrap();

    // Custom TYPE.
    if selection == COMMIT_TYPES.len() {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("GRC: What Type ?")
            .validate_with(|input: &str | -> Result<(), &str> {
                if input.len() == 0 || input.trim().len() == 0 {
                    Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap()
    } else {
        String::from(COMMIT_TYPES[selection])
    }
}

fn ask_scope() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("GRC: Scope ? (Optional)")
        .allow_empty(true)
        .interact()
        .unwrap()
}

fn ask_content() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("GRC: Commit Message ?")
        .validate_with(|input: &str | -> Result<(), &str> {
            if input.len() == 0 || input.trim().len() == 0 {
                Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap()
}
