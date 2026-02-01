//! Month representations with language support.

use chrono::{DateTime, Datelike, Months, NaiveTime, Utc};
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
pub enum January {
    #[default]
    January,
    #[cfg(feature = "swedish")]
    Januari,
}

impl WithLanguage for January {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Januari,
            Language::English(_) => Self::January,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum February {
    #[default]
    February,
    #[cfg(feature = "swedish")]
    Februari,
}

impl WithLanguage for February {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Februari,
            Language::English(_) => Self::February,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum March {
    #[default]
    March,
    #[cfg(feature = "swedish")]
    Mars,
}

impl WithLanguage for March {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Mars,
            Language::English(_) => Self::March,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum April {
    #[default]
    April,
}

impl WithLanguage for April {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::April,
            Language::English(_) => Self::April,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum May {
    #[default]
    May,
    #[cfg(feature = "swedish")]
    Maj,
}

impl WithLanguage for May {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Maj,
            Language::English(_) => Self::May,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum June {
    #[default]
    June,
    #[cfg(feature = "swedish")]
    Juni,
}

impl WithLanguage for June {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Juni,
            Language::English(_) => Self::June,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum July {
    #[default]
    July,
    #[cfg(feature = "swedish")]
    Juli,
}
impl WithLanguage for July {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Juli,
            Language::English(_) => Self::July,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum August {
    #[default]
    August,
    #[cfg(feature = "swedish")]
    Augusti,
}
impl WithLanguage for August {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Augusti,
            Language::English(_) => Self::August,
        }
    }
}
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum September {
    #[default]
    September,
}

impl WithLanguage for September {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::September,
            Language::English(_) => Self::September,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum October {
    #[default]
    October,
    #[cfg(feature = "swedish")]
    Oktober,
}

impl WithLanguage for October {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Oktober,
            Language::English(_) => Self::October,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum November {
    #[default]
    November,
}

impl WithLanguage for November {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::November,
            Language::English(_) => Self::November,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub enum December {
    #[default]
    December,
}

impl WithLanguage for December {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::December,
            Language::English(_) => Self::December,
        }
    }
}

/// A month with language-specific representations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Month {
    January(January),
    February(February),
    March(March),
    April(April),
    May(May),
    June(June),
    July(July),
    August(August),
    September(September),
    October(October),
    November(November),
    December(December),
}

impl WithLanguage for Month {
    fn with_language(&self, language: Language) -> Self {
        match self {
            Month::January(x) => Month::January(x.with_language(language)),
            Month::February(x) => Month::February(x.with_language(language)),
            Month::March(x) => Month::March(x.with_language(language)),
            Month::April(x) => Month::April(x.with_language(language)),
            Month::May(x) => Month::May(x.with_language(language)),
            Month::June(x) => Month::June(x.with_language(language)),
            Month::July(x) => Month::July(x.with_language(language)),
            Month::August(x) => Month::August(x.with_language(language)),
            Month::September(x) => Month::September(x.with_language(language)),
            Month::October(x) => Month::October(x.with_language(language)),
            Month::November(x) => Month::November(x.with_language(language)),
            Month::December(x) => Month::December(x.with_language(language)),
        }
    }
}

impl Month {
    pub fn january() -> Self {
        Self::January(January::default())
    }
    pub fn february() -> Self {
        Self::February(February::default())
    }
    pub fn march() -> Self {
        Self::March(March::default())
    }
    pub fn april() -> Self {
        Self::April(April::default())
    }
    pub fn may() -> Self {
        Self::May(May::default())
    }
    pub fn june() -> Self {
        Self::June(June::default())
    }
    pub fn july() -> Self {
        Self::July(July::default())
    }
    pub fn august() -> Self {
        Self::August(August::default())
    }
    pub fn september() -> Self {
        Self::September(September::default())
    }
    pub fn october() -> Self {
        Self::October(October::default())
    }
    pub fn november() -> Self {
        Self::November(November::default())
    }
    pub fn december() -> Self {
        Self::December(December::default())
    }
    /// Converts to a chrono month.
    pub fn to_chrono(self) -> chrono::Month {
        match self {
            Month::January(_) => chrono::Month::January,
            Month::February(_) => chrono::Month::February,
            Month::March(_) => chrono::Month::March,
            Month::April(_) => chrono::Month::April,
            Month::May(_) => chrono::Month::May,
            Month::June(_) => chrono::Month::June,
            Month::July(_) => chrono::Month::July,
            Month::August(_) => chrono::Month::August,
            Month::September(_) => chrono::Month::September,
            Month::October(_) => chrono::Month::October,
            Month::November(_) => chrono::Month::November,
            Month::December(_) => chrono::Month::December,
        }
    }

    /// Extracts the month from a timestamp in the specified language.
    ///
    /// When `first_midnight_means_month_before` is true, midnight on the first of the month
    /// is treated as belonging to the previous month.
    pub fn from_chrono(
        date_time: DateTime<Utc>,
        first_midnight_means_month_before: bool,
        language: Language,
    ) -> Self {
        let month_number = date_time.month();
        let month_number = if first_midnight_means_month_before
            && date_time.day() == 1
            && date_time.time() == NaiveTime::MIN
        {
            month_number - 1
        } else {
            month_number
        };

        match month_number {
            1 => Self::January(January::from_language(language)),
            2 => Self::February(February::from_language(language)),
            3 => Self::March(March::from_language(language)),
            4 => Self::April(April::from_language(language)),
            5 => Self::May(May::from_language(language)),
            6 => Self::June(June::from_language(language)),
            7 => Self::July(July::from_language(language)),
            8 => Self::August(August::from_language(language)),
            9 => Self::September(September::from_language(language)),
            10 => Self::October(October::from_language(language)),
            11 => Self::November(November::from_language(language)),
            12 | 0 => Self::December(December::from_language(language)),
            _ => unreachable!(),
        }
    }

    /// Converts to midnight on the first of the following month, relative to the given time.
    ///
    /// When `skip_self` is true, finds the next occurrence even if the current month matches.
    pub fn to_chrono_max(self, relative_to: DateTime<Utc>, skip_self: bool) -> DateTime<Utc> {
        let current_month = relative_to.month();
        let target_month = self.to_chrono().number_from_month();

        let difference = target_month as i8 - current_month as i8;
        let non_negative = (if difference < 0 {
            12 + difference
        } else {
            difference
        }) as u8;
        let skipped = if non_negative == 0 && skip_self {
            12
        } else {
            non_negative
        };

        let months_to_add = skipped as u32 + 1;

        relative_to
            .with_day(1)
            .unwrap()
            .checked_add_months(Months::new(months_to_add))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap()
    }
}
