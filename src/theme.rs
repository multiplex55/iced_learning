//! Small theme helpers.
//!
//! The goal is to keep visual decisions obvious and easy to tweak while the
//! sandbox is still young.

use iced::Theme;

/// Pure metadata for the theme demo.
///
/// This is intentionally separate from widget code so unit tests can verify the
/// theme-switching rules without rendering anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeChoice {
    Light,
    Dark,
    HighContrast,
}

impl ThemeChoice {
    pub const ALL: [Self; 3] = [Self::Light, Self::Dark, Self::HighContrast];

    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::HighContrast => "High contrast",
        }
    }

    pub fn teaching_note(self) -> &'static str {
        match self {
            Self::Light => "Useful when you want the default palette to stay easy to inspect.",
            Self::Dark => "Good for showing that one state flag can swap the whole app theme.",
            Self::HighContrast => {
                "Educational demo: accessibility-minded contrast presets are often modeled explicitly."
            }
        }
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark | Self::HighContrast)
    }

    pub fn to_iced_theme(self) -> Theme {
        match self {
            Self::Light => Theme::Light,
            Self::Dark => Theme::TokyoNight,
            Self::HighContrast => Theme::Ferra,
        }
    }
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// Returns the accent color used by summary cards and callouts.
pub fn accent_hex() -> &'static str {
    "#7C4DFF"
}

#[cfg(test)]
mod tests {
    use super::ThemeChoice;

    #[test]
    fn theme_choices_are_stable_and_distinct() {
        assert_eq!(
            ThemeChoice::ALL.map(ThemeChoice::label),
            ["Light", "Dark", "High contrast"]
        );
        assert!(!ThemeChoice::Light.is_dark());
        assert!(ThemeChoice::Dark.is_dark());
        assert!(ThemeChoice::HighContrast.is_dark());
    }
}
