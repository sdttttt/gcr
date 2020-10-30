use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::util::remove_pound_prefix;

const SPACE: &str = " ";

const COMMIT_TYPE_DESCRIPT: &[CommitTD] = &[
    CommitTD("test", "Adding missing tests."),
    CommitTD("feat", "A new feature."),
    CommitTD("fix", "A bug fix."),
    CommitTD("chore", "Build process or auxiliary tool changes."),
    CommitTD("docs", "Documentation only changes."),
    CommitTD("refactor", "A code change that neither fixes a bug or adds a feature."),
    CommitTD("style", "Markup, white-space, formatting, missing semi-colons..."),
    CommitTD("perf", "A code change that improves performance."),
    CommitTD("ci", "CI related changes."),
];

pub struct CommitTD(&'static str, &'static str);

// Messsager is Commit Message struct.
pub struct Messager {
    typ: String,
    scope: String,
    subject: String,
    body: String,
}

impl Messager {
    pub fn ask() -> Self {
        let typ = Self::ask_type();
        let scope = Self::ask_scope();
        let subject = Self::ask_subject();
        let body = Self::build_body();

        Self { typ, scope, subject, body }
    }

    // generate commit message.
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

    // generate commit long description.
    fn build_body() -> String {
        let description = Self::ask_description();
        let closes = Self::ask_close();

        let mut body = String::new();

        if description.len() != 0 {
            body = description;
        };

        if closes.len() != 0 {
            body = format!("{}\n\nClose #{}", body, closes);
        };

        body
    }

    // type of commit message.
    fn ask_type() -> String {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&Self::type_list())
            .default(0)
            .interact()
            .unwrap();

        // Custom TYPE.
        if selection == COMMIT_TYPE_DESCRIPT.len() {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("GRC: What Type ?")
                .validate_with(|input: &str| -> Result<(), &str> {
                    if input.len() == 0 || input.trim().len() == 0 {
                        Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
                    } else {
                        Ok(())
                    }
                })
                .interact()
                .unwrap()
        } else {
            String::from(COMMIT_TYPE_DESCRIPT[selection].0)
        }
    }

    // scope of commit scope.
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
            .validate_with(|input: &str| -> Result<(), &str> {
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

    fn type_list() -> Vec<String> {
        COMMIT_TYPE_DESCRIPT
            .iter()
            .map(|td: &CommitTD| -> String {
                let mut space = String::new();
                if td.0.len() < 12 {
                    for _ in 0..(11 - td.0.len()) {
                        space.push_str(SPACE);
                    }
                }
                format!("{}:{}{}", td.0, space, td.1)
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_type_list() {
        let result = Messager::type_list();

        assert_eq!(result[0].as_str(), "test:       Adding missing tests.")
    }
}
