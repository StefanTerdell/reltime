//! Traits for language-aware time types.

use crate::language::Language;

/// Converts a time type to a specific language representation.
pub trait WithLanguage {
    fn with_language(&self, language: Language) -> Self;
}

/// Creates a time type in a specific language from its default representation.
pub trait FromLanguage {
    fn from_language(language: Language) -> Self;
}

impl<T: Default + WithLanguage> FromLanguage for T {
    fn from_language(language: Language) -> Self {
        Self::default().with_language(language)
    }
}
