//! Pure helpers for the forms and validation demo.
//!
//! This module intentionally keeps validation deterministic so tests can verify
//! behavior without constructing UI widgets.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormDraft {
    pub name: String,
    pub email: String,
    pub goal: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    pub name: Option<&'static str>,
    pub email: Option<&'static str>,
    pub goal: Option<&'static str>,
}

impl ValidationErrors {
    pub fn is_valid(&self) -> bool {
        self.name.is_none() && self.email.is_none() && self.goal.is_none()
    }

    pub fn messages(&self) -> Vec<String> {
        [self.name, self.email, self.goal]
            .into_iter()
            .flatten()
            .map(str::to_owned)
            .collect()
    }
}

impl FormDraft {
    pub fn example() -> Self {
        Self {
            name: "Ava Learner".into(),
            email: "ava@example.com".into(),
            goal: "Build an async Iced dashboard with reusable page helpers.".into(),
        }
    }

    pub fn validate(&self) -> ValidationErrors {
        let name_trimmed = self.name.trim();
        let email_trimmed = self.email.trim();
        let goal_trimmed = self.goal.trim();

        ValidationErrors {
            name: if name_trimmed.len() < 2 {
                Some("Name should have at least 2 visible characters.")
            } else {
                None
            },
            email: if email_trimmed.contains('@') && email_trimmed.contains('.') {
                None
            } else {
                Some("Email should look like learner@example.com.")
            },
            goal: if goal_trimmed.len() < 12 {
                Some("Goal should explain what you want to build or learn next.")
            } else {
                None
            },
        }
    }

    pub fn submission_summary(&self) -> String {
        format!(
            "{} plans to {}",
            self.name.trim(),
            self.goal.trim().trim_end_matches('.')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FormDraft;

    #[test]
    fn example_data_is_internally_consistent() {
        let draft = FormDraft::example();
        let errors = draft.validate();

        assert!(errors.is_valid());
        assert!(draft.submission_summary().contains("plans to"));
    }

    #[test]
    fn validation_flags_each_invalid_field() {
        let draft = FormDraft {
            name: " ".into(),
            email: "invalid".into(),
            goal: "ship".into(),
        };

        let errors = draft.validate();
        assert!(!errors.is_valid());
        assert_eq!(errors.messages().len(), 3);
    }
}
