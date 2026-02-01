use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::traits::WithLanguage;

#[cfg(feature = "swedish")]
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Swedish {
    #[default]
    Swedish,
    Svenska,
}

#[cfg(feature = "swedish")]
impl WithLanguage for Swedish {
    fn with_language(&self, language: Language) -> Self {
        match language {
            Language::Swedish(_) => Self::Svenska,
            Language::English(_) => Self::Swedish,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum English {
    #[default]
    English,
    #[cfg(feature = "swedish")]
    Engelska,
}

impl WithLanguage for English {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Engelska,
            Language::English(_) => Self::English,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Language {
    English(English),
    #[cfg(feature = "swedish")]
    Swedish(Swedish),
}

impl Default for Language {
    fn default() -> Self {
        Self::English(English::default())
    }
}

impl WithLanguage for Language {
    fn with_language(&self, language: Language) -> Self {
        language
    }
}
