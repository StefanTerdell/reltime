use chrono::{DateTime, Days, Months, NaiveTime, Utc};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    exact::{ExactDate, ExactDateTime, ExactTime},
    language::Language,
    month::Month,
    traits::WithLanguage,
    weekday::{Sunday, Weekday},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
pub enum Today {
    #[default]
    Today,
    #[cfg(feature = "swedish")]
    Idag,
}

impl WithLanguage for Today {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Idag,
            Language::English(_) => Self::Today,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
pub enum Tomorrow {
    #[default]
    Tomorrow,
    #[cfg(feature = "swedish")]
    Imorgon,
}

impl WithLanguage for Tomorrow {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::Imorgon,
            Language::English(_) => Self::Tomorrow,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
pub enum ThisWeek {
    #[default]
    ThisWeek,
    #[cfg(feature = "swedish")]
    DennaVecka,
}

impl WithLanguage for ThisWeek {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::DennaVecka,
            Language::English(_) => Self::ThisWeek,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
pub enum NextWeek {
    #[default]
    NextWeek,
    #[cfg(feature = "swedish")]
    NästaVecka,
}

impl WithLanguage for NextWeek {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::NästaVecka,
            Language::English(_) => Self::NextWeek,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
pub enum ThisMonth {
    #[default]
    ThisMonth,
    #[cfg(feature = "swedish")]
    DennaMånad,
}

impl WithLanguage for ThisMonth {
    fn with_language(&self, language: Language) -> Self {
        match language {
            #[cfg(feature = "swedish")]
            Language::Swedish(_) => Self::DennaMånad,
            Language::English(_) => Self::ThisMonth,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Relative {
    Time(ExactTime),
    Date(ExactDate),
    DateTime(ExactDateTime),
    Today(Today),
    Tomorrow(Tomorrow),
    ThisWeek(ThisWeek),
    NextWeek(NextWeek),
    ThisMonth(ThisMonth),
}

impl WithLanguage for Relative {
    fn with_language(&self, language: Language) -> Self {
        match self {
            Relative::Time(x) => Relative::Time(*x),
            Relative::Date(x) => Relative::Date(*x),
            Relative::DateTime(x) => Relative::DateTime(*x),
            Relative::Today(x) => Relative::Today(x.with_language(language)),
            Relative::Tomorrow(x) => Relative::Tomorrow(x.with_language(language)),
            Relative::ThisWeek(x) => Relative::ThisWeek(x.with_language(language)),
            Relative::NextWeek(x) => Relative::NextWeek(x.with_language(language)),
            Relative::ThisMonth(x) => Relative::ThisMonth(x.with_language(language)),
        }
    }
}

impl Relative {
    pub fn today() -> Self {
        Self::Today(Today::default())
    }
    pub fn tomorrow() -> Self {
        Self::Tomorrow(Tomorrow::default())
    }
    pub fn this_week() -> Self {
        Self::ThisWeek(ThisWeek::default())
    }
    pub fn next_week() -> Self {
        Self::NextWeek(NextWeek::default())
    }
    pub fn this_month() -> Self {
        Self::ThisMonth(ThisMonth::default())
    }

    pub fn to_chrono_min_now(self) -> DateTime<Utc> {
        self.to_chrono_min(Utc::now())
    }

    pub fn to_chrono_min(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Relative::Time(x) => relative_to.with_time(x.to_chrono()).unwrap(),
            Relative::Date(x) => x
                .to_chrono_min(relative_to)
                .and_time(NaiveTime::MIN)
                .and_utc(),
            Relative::DateTime(x) => x.to_chrono_min(relative_to),
            Relative::Today(_) => relative_to.with_time(NaiveTime::MIN).unwrap(),
            Relative::Tomorrow(_) => relative_to
                .checked_add_days(Days::new(1))
                .unwrap()
                .with_time(NaiveTime::MIN)
                .unwrap(),
            Relative::ThisWeek(_) => Weekday::Sunday(Sunday::default())
                .to_chrono_max(relative_to.checked_sub_days(Days::new(7)).unwrap(), false),
            Relative::NextWeek(_) => {
                Weekday::Sunday(Sunday::default()).to_chrono_max(relative_to, false)
            }
            Relative::ThisMonth(_) => Month::from_chrono(relative_to, false, Language::default())
                .to_chrono_max(
                    relative_to.checked_sub_months(Months::new(1)).unwrap(),
                    false,
                ),
        }
    }

    pub fn to_chrono_max_now(self) -> DateTime<Utc> {
        self.to_chrono_max(Utc::now())
    }

    pub fn to_chrono_max(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Relative::Time(x) => {
                let x = x.to_chrono();

                if x < relative_to.time() {
                    relative_to
                        .checked_add_days(Days::new(1))
                        .unwrap()
                        .with_time(x)
                        .unwrap()
                } else {
                    relative_to.with_time(x).unwrap()
                }
            }
            Relative::Date(x) => x
                .to_chrono_max(relative_to)
                .and_time(NaiveTime::MIN)
                .and_utc(),
            Relative::DateTime(x) => x.to_chrono_max(relative_to),
            Relative::Today(_) => relative_to
                .checked_add_days(Days::new(1))
                .unwrap()
                .with_time(NaiveTime::MIN)
                .unwrap(),
            Relative::Tomorrow(_) => relative_to
                .checked_add_days(Days::new(2))
                .unwrap()
                .with_time(NaiveTime::MIN)
                .unwrap(),
            Relative::ThisWeek(_) => {
                Weekday::Sunday(Sunday::default()).to_chrono_max(relative_to, false)
            }
            Relative::NextWeek(_) => Weekday::Sunday(Sunday::default())
                .to_chrono_max(relative_to.checked_add_days(Days::new(7)).unwrap(), false),
            Relative::ThisMonth(_) => Month::from_chrono(relative_to, false, Language::default())
                .to_chrono_max(relative_to, false),
        }
    }
}
