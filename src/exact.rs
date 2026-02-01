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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn base_time() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2025-07-29T10:30:05-00:00")
            .unwrap()
            .to_utc()
    }

    #[test]
    fn exact_date_validation() {
        // Valid date
        let date = ExactDate::new(Some(2025), 7, 29);
        assert!(date.validate().is_ok());

        // Invalid month (clamped)
        let date = ExactDate::new(Some(2025), 13, 15);
        let validated = date.validated();
        assert_eq!(format!("{}", validated), "15/12/2025");

        // Invalid day (clamped)
        let date = ExactDate::new(Some(2025), 7, 35);
        let validated = date.validated();
        assert_eq!(format!("{}", validated), "31/7/2025");
    }

    #[test]
    fn exact_time_validation() {
        // Valid time
        let time = ExactTime::new(14, 30, Some(45));
        assert!(time.validate().is_ok());

        // Invalid hour (clamped)
        let time = ExactTime::new(25, 30, None);
        let validated = time.validated();
        assert_eq!(format!("{}", validated), "23:30");

        // Invalid minute (clamped)
        let time = ExactTime::new(14, 65, None);
        let validated = time.validated();
        assert_eq!(format!("{}", validated), "14:59");
    }

    #[test]
    fn exact_date_without_year_min() {
        let base = base_time(); // July 29th, 2025

        // Date in the past this year
        let march_15 = ExactDate::new(None, 3, 15);
        let chrono_min = march_15.to_chrono_min(base);

        // Should use current year even though it's in the past
        assert_eq!(chrono_min, NaiveDate::from_ymd_opt(2025, 3, 15).unwrap());
    }

    #[test]
    fn exact_date_without_year_max() {
        let base = base_time(); // July 29th, 2025

        // Date in the past this year
        let march_15 = ExactDate::new(None, 3, 15);
        let chrono_max = march_15.to_chrono_max(base);

        // Should use next year since we've already passed it
        assert_eq!(chrono_max, NaiveDate::from_ymd_opt(2026, 3, 15).unwrap());

        // Date in the future this year
        let december_25 = ExactDate::new(None, 12, 25);
        let chrono_max = december_25.to_chrono_max(base);

        // Should use current year
        assert_eq!(chrono_max, NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
    }

    #[test]
    fn exact_date_with_year() {
        let base = base_time();

        let date = ExactDate::new(Some(2026), 3, 15);

        let chrono_min = date.to_chrono_min(base);
        assert_eq!(chrono_min, NaiveDate::from_ymd_opt(2026, 3, 15).unwrap());

        let chrono_max = date.to_chrono_max(base);
        assert_eq!(chrono_max, NaiveDate::from_ymd_opt(2026, 3, 15).unwrap());
    }

    #[test]
    fn exact_time_accessors() {
        let time = ExactTime::new(14, 30, Some(45));
        assert_eq!(time.hour(), 14);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 45);

        let time_no_seconds = ExactTime::new(14, 30, None);
        assert_eq!(time_no_seconds.hour(), 14);
        assert_eq!(time_no_seconds.minute(), 30);
        assert_eq!(time_no_seconds.second(), 0);
    }

    #[test]
    fn exact_datetime_conversion() {
        let base = base_time();

        let date = ExactDate::new(Some(2025), 12, 25);
        let time = ExactTime::new(18, 30, None);
        let datetime = ExactDateTime::new(date, time);

        let chrono = datetime.to_chrono_min(base);
        assert_eq!(chrono.year(), 2025);
        assert_eq!(chrono.month(), 12);
        assert_eq!(chrono.day(), 25);
        assert_eq!(chrono.hour(), 18);
        assert_eq!(chrono.minute(), 30);
    }

    #[test]
    fn exact_date_roundtrip() {
        let original = NaiveDate::from_ymd_opt(2025, 7, 29).unwrap();
        let exact = ExactDate::from_chrono(original);
        let base = base_time();
        let converted = exact.to_chrono_min(base);
        assert_eq!(original, converted);
    }

    #[test]
    fn exact_time_roundtrip() {
        let original = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
        let exact = ExactTime::from_chrono(original);
        let converted = exact.to_chrono();
        assert_eq!(original, converted);
    }

    #[test]
    fn exact_date_year_boundary() {
        // Test date without year at year boundary
        let dec_31 = DateTime::parse_from_rfc3339("2025-12-31T23:59:59-00:00")
            .unwrap()
            .to_utc();

        // Jan 1 should be next year for max
        let jan_1 = ExactDate::new(None, 1, 1);
        let max = jan_1.to_chrono_max(dec_31);
        assert_eq!(max.year(), 2026);

        // Dec 31 should be this year for max (same day)
        let dec_31_date = ExactDate::new(None, 12, 31);
        let max = dec_31_date.to_chrono_max(dec_31);
        assert_eq!(max.year(), 2025);
    }

    #[test]
    fn exact_date_same_day() {
        let base = base_time(); // July 29th, 2025

        // Same date without year
        let july_29 = ExactDate::new(None, 7, 29);

        // Min should be this year
        let min = july_29.to_chrono_min(base);
        assert_eq!(min.year(), 2025);

        // Max should be this year (we haven't passed the entire day yet)
        let max = july_29.to_chrono_max(base);
        assert_eq!(max.year(), 2025);
    }

    #[test]
    fn exact_date_next_day() {
        let base = base_time(); // July 29th, 2025 at 10:30:05

        // Tomorrow's date without year
        let july_30 = ExactDate::new(None, 7, 30);

        // Max should still be this year (we haven't reached July 30 yet)
        let max = july_30.to_chrono_max(base);
        assert_eq!(max.year(), 2025);
        assert_eq!(max.month(), 7);
        assert_eq!(max.day(), 30);
    }

    #[test]
    fn exact_date_february_29_non_leap() {
        // Test Feb 29 in a non-leap year - should return default (epoch)
        let base = DateTime::parse_from_rfc3339("2025-03-01T00:00:00-00:00")
            .unwrap()
            .to_utc();

        let feb_29 = ExactDate::new(Some(2025), 2, 29);
        let result = feb_29.to_chrono_min(base);

        // from_ymd_opt returns None for invalid dates, unwrap_or_default gives epoch
        assert_eq!(result, NaiveDate::default());
    }
}
