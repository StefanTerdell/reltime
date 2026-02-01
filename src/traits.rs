use crate::language::Language;

pub trait WithLanguage {
    fn with_language(&self, language: Language) -> Self;
}

pub trait FromLanguage {
    fn from_language(language: Language) -> Self;
}

impl<T: Default + WithLanguage> FromLanguage for T {
    fn from_language(language: Language) -> Self {
        Self::default().with_language(language)
    }
}
