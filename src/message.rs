use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::util::remove_pound_prefix;

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
    subject: String,
    body: String
}

impl Messager {
    pub fn new() -> Self {
        let typ = ask_type();
        let scope = ask_scope();
        let subject = ask_subject();
        let body = Self::build_body();
        
        Self {
            typ,
            scope,
            subject,
            body
        }
    }

    pub fn build(&self) -> String {
        let header = if self.scope.trim().len() == 0 {
            format!("{}: {}", self.typ, self.subject)
        } else {
            format!("{}({}): {}", self.typ, self.scope, self.subject)
        };

        if self.body.trim().len() == 0 {
            header
        } else {
            format!("{} \n\n{}", header, self.body)
        }
    }

    fn build_body() -> String {
        let description = ask_description();
        let closes = ask_close();

        let mut body = String::new();

        if description.len() != 0 {
          body = description;
        };

        if closes.len() != 0 {
            body = format!("{}\n\nClose #{}", body, closes);
        };

        body
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
    let scope = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("GRC: Which scope? (Optional)")
        .allow_empty(true)
        .interact()
        .unwrap();

    String::from(scope.trim())
}

fn ask_subject() -> String {
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

fn ask_description() -> String {
    let description = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("GRC: Provide a longer description? (Optional)")
        .allow_empty(true)
        .interact()
        .unwrap();

    String::from(description.trim())
}

fn ask_close() -> String {
    let closes = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("GRC: PR & Issues this commit closes, e.g 123: (Optional)")
        .allow_empty(true)
        .interact()
        .unwrap();

    String::from(remove_pound_prefix(closes.trim()))
}