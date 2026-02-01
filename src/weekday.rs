//! Weekday representations with language support.

use chrono::{DateTime, Datelike, Days, NaiveTime, Utc};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    language::Language,
    traits::{FromLanguage, WithLanguage},
};

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Monday {
    #[default]
    Monday,
    #[cfg(feature = "swedish")]
    Måndag,
}

impl WithLanguage for Monday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Måndag,
            Language::English(_) => Self::Monday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Tuesday {
    #[default]
    Tuesday,
    #[cfg(feature = "swedish")]
    Tisdag,
}

impl WithLanguage for Tuesday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Tisdag,
            Language::English(_) => Self::Tuesday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Wednesday {
    #[default]
    Wednesday,
    #[cfg(feature = "swedish")]
    Onsdag,
}

impl WithLanguage for Wednesday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Onsdag,
            Language::English(_) => Self::Wednesday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Thursday {
    #[default]
    Thursday,
    #[cfg(feature = "swedish")]
    Torsdag,
}

impl WithLanguage for Thursday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Torsdag,
            Language::English(_) => Self::Thursday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Friday {
    #[default]
    Friday,
    #[cfg(feature = "swedish")]
    Fredag,
}

impl WithLanguage for Friday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Fredag,
            Language::English(_) => Self::Friday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Saturday {
    #[default]
    Saturday,
    #[cfg(feature = "swedish")]
    Lördag,
}

impl WithLanguage for Saturday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Lördag,
            Language::English(_) => Self::Saturday,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum Sunday {
    #[default]
    Sunday,
    #[cfg(feature = "swedish")]
    Söndag,
}

impl WithLanguage for Sunday {
    fn with_language(&self, langue: Language) -> Self {
        match langue {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Söndag,
            Language::English(_) => Self::Sunday,
        }
    }
}

/// A weekday with language-specific representations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Weekday {
    Monday(Monday),
    Tuesday(Tuesday),
    Wednesday(Wednesday),
    Thursday(Thursday),
    Friday(Friday),
    Saturday(Saturday),
    Sunday(Sunday),
}

impl WithLanguage for Weekday {
    fn with_language(&self, language: Language) -> Self {
        match self {
            Weekday::Monday(x) => Weekday::Monday(x.with_language(language)),
            Weekday::Tuesday(x) => Weekday::Tuesday(x.with_language(language)),
            Weekday::Wednesday(x) => Weekday::Wednesday(x.with_language(language)),
            Weekday::Thursday(x) => Weekday::Thursday(x.with_language(language)),
            Weekday::Friday(x) => Weekday::Friday(x.with_language(language)),
            Weekday::Saturday(x) => Weekday::Saturday(x.with_language(language)),
            Weekday::Sunday(x) => Weekday::Sunday(x.with_language(language)),
        }
    }
}

impl Weekday {
    pub fn monday() -> Self {
        Self::Monday(Monday::default())
    }
    pub fn tuesday() -> Self {
        Self::Tuesday(Tuesday::default())
    }
    pub fn wednesday() -> Self {
        Self::Wednesday(Wednesday::default())
    }
    pub fn thursday() -> Self {
        Self::Thursday(Thursday::default())
    }
    pub fn friday() -> Self {
        Self::Friday(Friday::default())
    }
    pub fn saturday() -> Self {
        Self::Saturday(Saturday::default())
    }
    pub fn sunday() -> Self {
        Self::Sunday(Sunday::default())
    }
    /// Converts to a chrono weekday.
    pub fn to_chrono(self) -> chrono::Weekday {
        match self {
            Weekday::Monday(_) => chrono::Weekday::Mon,
            Weekday::Tuesday(_) => chrono::Weekday::Tue,
            Weekday::Wednesday(_) => chrono::Weekday::Wed,
            Weekday::Thursday(_) => chrono::Weekday::Thu,
            Weekday::Friday(_) => chrono::Weekday::Fri,
            Weekday::Saturday(_) => chrono::Weekday::Sat,
            Weekday::Sunday(_) => chrono::Weekday::Sun,
        }
    }

    /// Extracts the weekday from a timestamp in the specified language.
    ///
    /// When `midnight_means_day_before` is true, midnight timestamps are treated
    /// as belonging to the previous day.
    pub fn from_chrono(
        date_time: DateTime<Utc>,
        midnight_means_day_before: bool,
        language: Language,
    ) -> Self {
        let weekday_number = date_time.weekday().number_from_monday();

        let weekday_number = if midnight_means_day_before && date_time.time() == NaiveTime::MIN {
            weekday_number - 1
        } else {
            weekday_number
        };

        match weekday_number {
            1 => Self::Monday(Monday::from_language(language)),
            2 => Self::Tuesday(Tuesday::from_language(language)),
            3 => Self::Wednesday(Wednesday::from_language(language)),
            4 => Self::Thursday(Thursday::from_language(language)),
            5 => Self::Friday(Friday::from_language(language)),
            6 => Self::Saturday(Saturday::from_language(language)),
            7 | 0 => Self::Sunday(Sunday::from_language(language)),
            _ => unreachable!(),
        }
    }

    /// Converts to the earliest timestamp for this weekday, relative to the given time.
    ///
    /// When `skip_self` is true, finds the next occurrence even if the current day matches.
    pub fn to_chrono_min(self, relative_to: DateTime<Utc>, skip_self: bool) -> DateTime<Utc> {
        self.to_chrono_max(relative_to, skip_self)
            .checked_sub_days(Days::new(1))
            .unwrap()
            .max(relative_to)
    }

    /// Converts to midnight after this weekday, relative to the given time.
    ///
    /// When `skip_self` is true, finds the next occurrence even if the current day matches.
    pub fn to_chrono_max(self, relative_to: DateTime<Utc>, skip_self: bool) -> DateTime<Utc> {
        let current_weekday = relative_to.weekday().number_from_monday();
        let target_weekday = self.to_chrono().number_from_monday();
        let difference = target_weekday as i8 - current_weekday as i8;
        let non_negative = (if difference < 0 {
            7 + difference
        } else {
            difference
        }) as u8;

        let skipped = if non_negative == 0 && skip_self {
            7
        } else {
            non_negative
        };

        let days_to_add = skipped as u64 + 1;

        relative_to
            .checked_add_days(Days::new(days_to_add))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap()
    }
}
