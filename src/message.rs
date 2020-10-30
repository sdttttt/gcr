use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::util::remove_pound_prefix;

const SPACE: &str = " ";

const BASE_COMMIT_TYPE_DESCRIPTION: &[CommitTD] = &[
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

#[derive(Clone)]
pub struct CommitTD(pub &'static str, pub &'static str);

// Messsager is Commit Message struct.
pub struct Messager {
    commit_type_descript: Vec<CommitTD>,

    typ: String,
    scope: String,
    subject: String,
    body: String,
}

impl Messager {
    pub fn new() -> Self {
        let commit_type_descript = vec![];
        let typ = String::new();
        let scope = String::new();
        let subject = String::new();
        let body = String::new();

        Self { commit_type_descript, typ, scope, subject, body }
    }

    pub fn load_ext_td(mut self, td: Vec<CommitTD>) -> Self {
        self.commit_type_descript = [BASE_COMMIT_TYPE_DESCRIPTION, &td].concat();
        self
    }

    pub fn ask(mut self) -> Self {
        self.typ = Self::ask_type();
        self.scope = Self::ask_scope();
        self.subject = Self::ask_subject();
        self.body = Self::build_body();
        self
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
        if selection == BASE_COMMIT_TYPE_DESCRIPTION.len() {
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
            String::from(BASE_COMMIT_TYPE_DESCRIPTION[selection].0)
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
        BASE_COMMIT_TYPE_DESCRIPTION
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
