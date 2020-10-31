use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::util::remove_pound_prefix;

const SPACE: &str = " ";

const BASE_COMMIT_TYPE_DESCRIPTION: &[(&str, &str)] = &[
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

struct CommitTD(String, String);

// Messsager is Commit Message struct.
pub struct Messager {
    commit_type_descript: Vec<CommitTD>,

    typ: String,
    scope: String,
    subject: String,
    body: String,
}

impl CommitTD {
    pub fn from(a1: &str, a2: &str) -> Self {
        Self(a1.to_string(), a2.to_string())
    }
}

impl Messager {
    pub fn new() -> Self {
        let commit_type_descript = BASE_COMMIT_TYPE_DESCRIPTION
            .iter()
            .map(|td: &(&str, &str)| CommitTD::from(td.0, td.1))
            .collect();
        let typ = String::new();
        let scope = String::new();
        let subject = String::new();
        let body = String::new();

        Self { commit_type_descript, typ, scope, subject, body }
    }

    pub fn load_ext_td(mut self, t: &Vec<String>) -> Self {
        let mut td = t
            .iter()
            .map(|typ: &String| -> CommitTD {
                let arr_td = typ.split(":").collect::<Vec<&str>>();
                CommitTD::from(arr_td[0], arr_td[1])
            })
            .collect::<Vec<CommitTD>>();
        self.commit_type_descript.append(&mut td);
        self.append_custom_td();
        self
    }

    pub fn ask(mut self) -> Self {
        self.ask_type();
        self.ask_scope();
        self.ask_subject();
        self.build_body();
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

    fn append_custom_td(&mut self) {
        self.commit_type_descript.push(CommitTD::from("<~>", "Define your commit type."))
    }

    // generate commit long description.
    fn build_body(&mut self) {
        let description = self.ask_description();
        let closes = self.ask_close();

        let mut body = String::new();

        if description.len() > 0 {
            body = description;
        };

        if closes.len() > 0 {
            body = format!("{}\n\nClose #{}", body, closes);
        };

        self.body = body
    }

    // type of commit message.
    fn ask_type(&mut self) {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&self.type_list())
            .default(0)
            .interact()
            .unwrap();

        // Custom TYPE.
        if selection == BASE_COMMIT_TYPE_DESCRIPTION.len() {
            self.typ = Input::with_theme(&ColorfulTheme::default())
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
            self.typ = String::from(BASE_COMMIT_TYPE_DESCRIPTION[selection].0)
        }
    }

    // scope of commit scope.
    fn ask_scope(&mut self) {
        let scope = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("GRC: Which scope? (Optional)")
            .allow_empty(true)
            .interact()
            .unwrap();

        self.scope = String::from(scope.trim())
    }

    fn ask_subject(&mut self) {
        self.subject = Input::<String>::with_theme(&ColorfulTheme::default())
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

    fn ask_description(&self) -> String {
        let description = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("GRC: Provide a longer description? (Optional)")
            .allow_empty(true)
            .interact()
            .unwrap();

        String::from(description.trim())
    }

    fn ask_close(&self) -> String {
        let closes = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("GRC: PR & Issues this commit closes, e.g 123: (Optional)")
            .allow_empty(true)
            .interact()
            .unwrap();

        String::from(remove_pound_prefix(closes.trim()))
    }

    fn type_list(&self) -> Vec<String> {
        self.commit_type_descript
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
mod tests {}
