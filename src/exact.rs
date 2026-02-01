//! Exact date and time types with validation.

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactYear(i16);

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactMonth(#[schemars(range(min = 1, max = 12))] u8);

impl ExactMonth {
    pub fn validate(self) -> Result<Self, Self> {
        let valid = self.0.clamp(1, 12);

        if self.0 == valid {
            Ok(self)
        } else {
            Err(Self(valid))
        }
    }

    pub fn new(month: u8) -> Self {
        Self(month).validated()
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactDay(#[schemars(range(min = 1, max = 31))] u8);

impl ExactDay {
    pub fn validate(self) -> Result<Self, Self> {
        let valid = self.0.clamp(1, 31);

        if self.0 == valid {
            Ok(self)
        } else {
            Err(Self(valid))
        }
    }
}
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactHour(#[schemars(range(min = 0, max = 23))] u8);

impl ExactHour {
    pub fn validate(self) -> Result<Self, Self> {
        let valid = self.0.clamp(0, 23);

        if self.0 == valid {
            Ok(self)
        } else {
            Err(Self(valid))
        }
    }

    pub fn new(hour: u8) -> Self {
        Self(hour).validated()
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactMinute(#[schemars(range(min = 0, max = 59))] u8);

impl ExactMinute {
    pub fn validate(self) -> Result<Self, Self> {
        let valid = self.0.clamp(0, 59);

        if self.0 == valid {
            Ok(self)
        } else {
            Err(Self(valid))
        }
    }
    pub fn new(minute: u8) -> Self {
        Self(minute).validated()
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display,
)]
pub struct ExactSecond(#[schemars(range(min = 0, max = 59))] u8);

impl ExactSecond {
    pub fn validate(self) -> Result<Self, Self> {
        let valid = self.0.clamp(0, 59);

        if self.0 == valid {
            Ok(self)
        } else {
            Err(Self(valid))
        }
    }
    pub fn new(second: u8) -> Self {
        Self(second).validated()
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}

/// A calendar date, optionally without a year for recurring dates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum ExactDate {
    WithYear(ExactYear, ExactMonth, ExactDay),
    WithoutYear(ExactMonth, ExactDay),
}

impl Display for ExactDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExactDate::WithYear(y, m, d) => f.write_fmt(format_args!("{d}/{m}/{y}")),
            ExactDate::WithoutYear(m, d) => f.write_fmt(format_args!("{d}/{m}")),
        }
    }
}

impl ExactDate {
    pub fn validate(self) -> Result<Self, Self> {
        match self {
            ExactDate::WithYear(y, m, d) => match (y, m.validate(), d.validate()) {
                (y, Ok(m), Ok(d)) => Ok(Self::WithYear(y, m, d)),
                (y, Ok(m), Err(d)) | (y, Err(m), Ok(d)) | (y, Err(m), Err(d)) => {
                    Err(Self::WithYear(y, m, d))
                }
            },
            ExactDate::WithoutYear(m, d) => match (m.validate(), d.validate()) {
                (Ok(m), Ok(d)) => Ok(Self::WithoutYear(m, d)),
                (Ok(m), Err(d)) | (Err(m), Ok(d)) | (Err(m), Err(d)) => {
                    Err(Self::WithoutYear(m, d))
                }
            },
        }
    }

    pub fn from_chrono(x: NaiveDate) -> Self {
        Self::WithYear(
            ExactYear(x.year() as i16),
            ExactMonth((x.month0() + 1) as u8),
            ExactDay((x.day0() + 1) as u8),
        )
    }
    pub fn to_chrono_min(&self, relative_to: DateTime<Utc>) -> NaiveDate {
        let (year, month, day) = match self {
            ExactDate::WithYear(y, m, d) => (y.0 as i32, m.0, d.0),
            ExactDate::WithoutYear(m, d) => (relative_to.year(), m.0, d.0),
        };

        NaiveDate::from_ymd_opt(year, month.into(), day.into()).unwrap_or_default()
    }
    pub fn to_chrono_max(&self, relative_to: DateTime<Utc>) -> NaiveDate {
        let (year, month, day) = match self {
            ExactDate::WithYear(y, m, d) => (y.0 as i32, m.0, d.0),
            ExactDate::WithoutYear(m, d) => {
                let year = if relative_to.month() > m.0 as u32
                    || (relative_to.month() == m.0 as u32 && relative_to.day() > d.0 as u32)
                {
                    relative_to.year() + 1
                } else {
                    relative_to.year()
                };

                (year, m.0, d.0)
            }
        };

        NaiveDate::from_ymd_opt(year, month.into(), day.into()).unwrap_or_default()
    }

    pub fn new(year: Option<i16>, month: u8, day: u8) -> Self {
        match year {
            Some(year) => {
                Self::WithYear(ExactYear(year), ExactMonth(month), ExactDay(day)).validated()
            }
            None => Self::WithoutYear(ExactMonth(month), ExactDay(day)).validated(),
        }
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}

/// A time of day, optionally without seconds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum ExactTime {
    WithSecond(ExactHour, ExactMinute, ExactSecond),
    WithoutSecond(ExactHour, ExactMinute),
}

impl ExactTime {
    pub fn new(hour: u8, minute: u8, second: Option<u8>) -> Self {
        match second {
            Some(second) => {
                Self::WithSecond(ExactHour(hour), ExactMinute(minute), ExactSecond(second))
                    .validated()
            }
            None => Self::WithoutSecond(ExactHour(hour), ExactMinute(minute)).validated(),
        }
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }

    pub fn validate(self) -> Result<Self, Self> {
        match self {
            ExactTime::WithSecond(hour, minute, second) => {
                match (hour.validate(), minute.validate(), second.validate()) {
                    (Ok(h), Ok(m), Ok(s)) => Ok(Self::WithSecond(h, m, s)),
                    (Ok(h), Ok(m), Err(s))
                    | (Ok(h), Err(m), Ok(s))
                    | (Ok(h), Err(m), Err(s))
                    | (Err(h), Ok(m), Ok(s))
                    | (Err(h), Ok(m), Err(s))
                    | (Err(h), Err(m), Ok(s))
                    | (Err(h), Err(m), Err(s)) => Err(Self::WithSecond(h, m, s)),
                }
            }
            ExactTime::WithoutSecond(hour, minute) => match (hour.validate(), minute.validate()) {
                (Ok(h), Ok(m)) => Ok(Self::WithoutSecond(h, m)),
                (Ok(h), Err(m)) | (Err(h), Ok(m)) | (Err(h), Err(m)) => {
                    Err(Self::WithoutSecond(h, m))
                }
            },
        }
    }

    pub fn hour(&self) -> u8 {
        match self {
            ExactTime::WithSecond(hour, ..) | ExactTime::WithoutSecond(hour, ..) => hour.0,
        }
    }

    pub fn minute(&self) -> u8 {
        match self {
            ExactTime::WithSecond(_, minute, ..) | ExactTime::WithoutSecond(_, minute, ..) => {
                minute.0
            }
        }
    }

    pub fn second(&self) -> u8 {
        match self {
            ExactTime::WithSecond(_, _, second) => second.0,
            ExactTime::WithoutSecond(..) => 0,
        }
    }

    pub fn from_chrono(x: NaiveTime) -> Self {
        Self::WithSecond(
            ExactHour(x.hour() as u8),
            ExactMinute(x.minute() as u8),
            ExactSecond(x.second() as u8),
        )
    }

    pub fn to_chrono(&self) -> NaiveTime {
        let (h, m, s) = match self {
            ExactTime::WithSecond(h, m, s) => (h.0, m.0, s.0),
            ExactTime::WithoutSecond(h, m) => (h.0, m.0, 0),
        };

        NaiveTime::from_hms_opt(h.into(), m.into(), s.into()).unwrap_or_default()
    }
}

impl Display for ExactTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExactTime::WithSecond(hour, minute, second) => {
                f.write_fmt(format_args!("{hour}:{minute}:{second}"))
            }
            ExactTime::WithoutSecond(hour, minute) => f.write_fmt(format_args!("{hour}:{minute}")),
        }
    }
}

/// A combination of date and time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[display("{} {}", self.0, self.1)]
pub struct ExactDateTime(ExactDate, ExactTime);

impl ExactDateTime {
    pub fn new(date: ExactDate, time: ExactTime) -> Self {
        Self(date.validated(), time.validated())
    }

    pub fn validate(self) -> Result<Self, Self> {
        match (self.0.validate(), self.1.validate()) {
            (Ok(m), Ok(d)) => Ok(Self(m, d)),
            (Ok(m), Err(d)) | (Err(m), Ok(d)) | (Err(m), Err(d)) => Err(Self(m, d)),
        }
    }

    pub fn to_chrono_min(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        NaiveDateTime::new(self.0.to_chrono_min(relative_to), self.1.to_chrono()).and_utc()
    }

    pub fn to_chrono_max(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        NaiveDateTime::new(self.0.to_chrono_max(relative_to), self.1.to_chrono()).and_utc()
    }

    pub fn validated(self) -> Self {
        match self.validate() {
            Ok(x) | Err(x) => x,
        }
    }
}
