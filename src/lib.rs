//! Relative time representations with language support.
//!
//! This library provides types for expressing dates and times in human-readable forms,
//! supporting both absolute and relative representations. All types serialise to and from
//! JSON in a natural format.
//!
//! # Features
//!
//! - `swedish` (default): Enables Swedish language variants for all time types.

use chrono::{DateTime, Months, NaiveTime, Utc};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    exact::ExactDateTime,
    language::Language,
    month::{
        April, August, December, February, January, July, June, March, May, Month, November,
        October, September,
    },
    relative::{Relative, ThisMonth, ThisWeek, Today, Tomorrow},
    traits::FromLanguage,
    weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday, Weekday},
};

pub mod exact;
pub mod language;
pub mod month;
pub mod relative;
pub mod traits;
pub mod weekday;

/// A time representation supporting relative, named, exact, and absolute forms.
///
/// Serialises as an untagged enum, allowing natural JSON representations like
/// `"Today"`, `"Monday"`, `"2025-07-29T10:30:05Z"`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Time {
    Relative(Relative),
    Weekday(Weekday),
    Month(Month),
    Exact(ExactDateTime),
    DateTime(DateTime<Utc>),
}

impl Time {
    /// Converts to the earliest possible timestamp, relative to the current time.
    pub fn to_chrono_min_now(self) -> DateTime<Utc> {
        self.to_chrono_min(Utc::now())
    }

    /// Converts to the earliest possible timestamp, relative to the given time.
    pub fn to_chrono_min(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Time::Relative(relative) => relative.to_chrono_min(relative_to),
            Time::Weekday(weekday) => weekday.to_chrono_min(relative_to, true),
            Time::Month(month) => month
                .to_chrono_max(relative_to, true)
                .checked_sub_months(Months::new(1))
                .unwrap(),
            Time::Exact(exact) => exact.to_chrono_min(relative_to),
            Time::DateTime(date_time) => date_time,
        }
    }

    /// Converts to the latest possible timestamp, relative to the current time.
    pub fn to_chrono_max_now(self) -> DateTime<Utc> {
        self.to_chrono_max(Utc::now())
    }

    /// Converts to the latest possible timestamp, relative to the given time.
    pub fn to_chrono_max(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Time::Relative(relative) => relative.to_chrono_max(relative_to),
            Time::Weekday(weekday) => weekday.to_chrono_max(relative_to, true),
            Time::Month(month) => month.to_chrono_max(relative_to, true),
            Time::Exact(exact) => exact.to_chrono_max(relative_to),
            Time::DateTime(date_time) => date_time,
        }
    }

    /// Converts a chrono timestamp to the most natural time representation.
    ///
    /// When `relative_to` is provided, attempts to express the timestamp as a relative
    /// or named time (e.g., "Today", "Monday") in the specified language.
    pub fn from_max_chrono(
        date_time: DateTime<Utc>,
        relative_to: Option<DateTime<Utc>>,
        language: Language,
    ) -> Time {
        if let Some(now) = relative_to
            && date_time.time() == NaiveTime::MIN
        {
            let today = Relative::Today(Today::from_language(language));
            if date_time == today.clone().to_chrono_max(now) {
                return Time::Relative(today);
            };

            let tomorrow = Relative::Tomorrow(Tomorrow::from_language(language));
            if date_time == tomorrow.clone().to_chrono_max(now) {
                return Time::Relative(tomorrow);
            }

            let monday = Time::Weekday(Weekday::Monday(Monday::from_language(language)));
            if date_time == monday.clone().to_chrono_max(now) {
                return monday;
            }

            let tuesday = Time::Weekday(Weekday::Tuesday(Tuesday::from_language(language)));
            if date_time == tuesday.clone().to_chrono_max(now) {
                return tuesday;
            }

            let wednesday = Time::Weekday(Weekday::Wednesday(Wednesday::from_language(language)));
            if date_time == wednesday.clone().to_chrono_max(now) {
                return wednesday;
            }

            let thursday = Time::Weekday(Weekday::Thursday(Thursday::from_language(language)));
            if date_time == thursday.clone().to_chrono_max(now) {
                return thursday;
            }

            let friday = Time::Weekday(Weekday::Friday(Friday::from_language(language)));
            if date_time == friday.clone().to_chrono_max(now) {
                return friday;
            }

            let saturday = Time::Weekday(Weekday::Saturday(Saturday::from_language(language)));
            if date_time == saturday.clone().to_chrono_max(now) {
                return saturday;
            }

            let sunday = Time::Weekday(Weekday::Sunday(Sunday::from_language(language)));
            if date_time == sunday.clone().to_chrono_max(now) {
                return sunday;
            }

            let january = Time::Month(Month::January(January::from_language(language)));
            if date_time == january.clone().to_chrono_max(now) {
                return january;
            }

            let february = Time::Month(Month::February(February::from_language(language)));
            if date_time == february.clone().to_chrono_max(now) {
                return february;
            }

            let march = Time::Month(Month::March(March::from_language(language)));
            if date_time == march.clone().to_chrono_max(now) {
                return march;
            }

            let april = Time::Month(Month::April(April::from_language(language)));
            if date_time == april.clone().to_chrono_max(now) {
                return april;
            }

            let may = Time::Month(Month::May(May::from_language(language)));
            if date_time == may.clone().to_chrono_max(now) {
                return may;
            }

            let june = Time::Month(Month::June(June::from_language(language)));
            if date_time == june.clone().to_chrono_max(now) {
                return june;
            }

            let july = Time::Month(Month::July(July::from_language(language)));
            if date_time == july.clone().to_chrono_max(now) {
                return july;
            }

            let august = Time::Month(Month::August(August::from_language(language)));
            if date_time == august.clone().to_chrono_max(now) {
                return august;
            }

            let september = Time::Month(Month::September(September::from_language(language)));
            if date_time == september.clone().to_chrono_max(now) {
                return september;
            }

            let october = Time::Month(Month::October(October::from_language(language)));
            if date_time == october.clone().to_chrono_max(now) {
                return october;
            }

            let november = Time::Month(Month::November(November::from_language(language)));
            if date_time == november.clone().to_chrono_max(now) {
                return november;
            }

            let december = Time::Month(Month::December(December::from_language(language)));
            if date_time == december.clone().to_chrono_max(now) {
                return december;
            }

            let this_week = Relative::ThisWeek(ThisWeek::from_language(language));
            if date_time == this_week.clone().to_chrono_max(now) {
                return Time::Relative(this_week);
            }

            let this_month = Relative::ThisMonth(ThisMonth::from_language(language));
            if date_time == this_month.clone().to_chrono_max(now) {
                return Time::Relative(this_month);
            }
        }

        Time::DateTime(date_time)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Days};

    use super::*;

    /// Creates a DateTime\<Utc> for Tuesday July 29th, 2025 at 10:30:05.
    fn base_time() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2025-07-29T10:30:05-00:00")
            .unwrap()
            .to_utc()
    }

    #[test]
    fn weekday_next_midnight() {
        let tuesday = base_time();

        let weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(weekday, Weekday::tuesday());

        let midnight_tomorrow = weekday.to_chrono_max(tuesday, false);

        assert_eq!(
            midnight_tomorrow,
            tuesday
                .checked_add_days(Days::new(1))
                .unwrap()
                .with_time(NaiveTime::MIN)
                .unwrap()
        );
    }

    #[test]
    fn weekday_next_midnight_skipping_self() {
        let tuesday = base_time();

        let tuesday_as_weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(tuesday_as_weekday, Weekday::tuesday());

        let midnight_next_wednesday = tuesday_as_weekday.to_chrono_max(tuesday, true);

        assert_eq!(
            midnight_next_wednesday,
            tuesday
                .checked_add_days(Days::new(8))
                .unwrap()
                .with_time(NaiveTime::MIN)
                .unwrap()
        );
    }

    #[test]
    fn midnight_tomorrow_means_today() {
        let tuesday = base_time();

        let midnight_tomorrow = tuesday
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        let time = Time::from_max_chrono(midnight_tomorrow, Some(tuesday), Language::default());

        assert_eq!(time, Time::Relative(Relative::today()));
    }

    #[test]
    fn midnight_in_two_days_means_tomorrow() {
        let tuesday = base_time();

        let midnight_in_two_days = tuesday
            .checked_add_days(Days::new(2))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        let time = Time::from_max_chrono(midnight_in_two_days, Some(tuesday), Language::default());

        assert_eq!(time, Time::Relative(Relative::tomorrow()));
    }

    #[test]
    fn weekday_chrono_min() {
        let tuesday = base_time();

        let tuesday_as_weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(tuesday_as_weekday, Weekday::tuesday());

        let tuesday_as_weekday_min = tuesday_as_weekday.to_chrono_min(tuesday, false);

        let tuesday_min = tuesday;

        assert_eq!(tuesday_min, tuesday_as_weekday_min);
    }

    #[test]
    fn weekday_chrono_min_skipping_self() {
        let tuesday = base_time();

        let tuesday_as_weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(tuesday_as_weekday, Weekday::tuesday());

        let next_tuesday_as_weekday_min = tuesday_as_weekday.to_chrono_min(tuesday, true);

        let next_tuesday_min = tuesday
            .checked_add_days(Days::new(7))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        assert_eq!(next_tuesday_min, next_tuesday_as_weekday_min);
    }

    #[test]
    fn weekday_chrono_max() {
        let tuesday = base_time();

        let tuesday_as_weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(tuesday_as_weekday, Weekday::tuesday());

        let tuesday_as_weekday_max = tuesday_as_weekday.to_chrono_max(tuesday, false);

        let tuesday_max = tuesday
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        assert_eq!(tuesday_max, tuesday_as_weekday_max);
    }

    #[test]
    fn weekday_chrono_max_skipping_self() {
        let tuesday = base_time();

        let tuesday_as_weekday = Weekday::from_chrono(tuesday, false, Language::default());

        assert_eq!(tuesday_as_weekday, Weekday::tuesday());

        let tuesday_as_weekday_max = tuesday_as_weekday.to_chrono_max(tuesday, true);

        let tuesday_max = tuesday
            .checked_add_days(Days::new(8))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        assert_eq!(tuesday_max, tuesday_as_weekday_max);
    }

    #[test]
    fn midnight_next_monday_means_sunday() {
        let tuesday = base_time();

        dbg!(&tuesday);
        let midnight_next_monday = tuesday
            .checked_add_days(Days::new(6))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        dbg!(&midnight_next_monday);

        assert_eq!(
            midnight_next_monday.weekday(),
            chrono::Weekday::Mon,
            "chrono monday check"
        );

        let time = Time::from_max_chrono(midnight_next_monday, Some(tuesday), Language::default());

        println!(
            "sunday chrono max {}",
            Time::Weekday(Weekday::sunday()).to_chrono_max(tuesday)
        );

        assert_eq!(time, Time::Weekday(Weekday::sunday()));
    }

    #[test]
    fn month_conversions() {
        let tuesday = base_time(); // July 29th, 2025

        let july = Month::from_chrono(tuesday, false, Language::default());
        assert_eq!(july, Month::july());

        // Next occurrence of July should be midnight on August 1st
        let next_july = july.to_chrono_max(tuesday, false);
        assert_eq!(next_july.month(), 8);
        assert_eq!(next_july.day(), 1);
    }

    #[test]
    fn month_skipping_self() {
        let tuesday = base_time(); // July 29th, 2025

        let july = Month::july();

        // Skipping self should give us next year's July
        let next_july = july.to_chrono_max(tuesday, true);
        assert_eq!(next_july.year(), 2026);
        assert_eq!(next_july.month(), 8);
        assert_eq!(next_july.day(), 1);
    }

    #[test]
    fn relative_today_conversion() {
        let tuesday = base_time();

        let today = Relative::today();

        // Min should be start of today
        let min = today.clone().to_chrono_min(tuesday);
        assert_eq!(min.date_naive(), tuesday.date_naive());
        assert_eq!(min.time(), NaiveTime::MIN);

        // Max should be start of tomorrow
        let max = today.to_chrono_max(tuesday);
        assert_eq!(max, tuesday.checked_add_days(Days::new(1)).unwrap().with_time(NaiveTime::MIN).unwrap());
    }

    #[test]
    fn relative_tomorrow_conversion() {
        let tuesday = base_time();

        let tomorrow = Relative::tomorrow();

        // Min should be start of tomorrow
        let min = tomorrow.clone().to_chrono_min(tuesday);
        assert_eq!(min, tuesday.checked_add_days(Days::new(1)).unwrap().with_time(NaiveTime::MIN).unwrap());

        // Max should be start of day after tomorrow
        let max = tomorrow.to_chrono_max(tuesday);
        assert_eq!(max, tuesday.checked_add_days(Days::new(2)).unwrap().with_time(NaiveTime::MIN).unwrap());
    }

    #[test]
    #[cfg(feature = "swedish")]
    fn language_switching_weekday() {
        use crate::traits::WithLanguage;
        use crate::language::{Language, Swedish};

        let monday_english = Monday::default();
        assert_eq!(format!("{}", monday_english), "Monday");

        let monday_swedish = monday_english.with_language(Language::Swedish(Swedish::default()));
        assert_eq!(format!("{}", monday_swedish), "Måndag");
    }

    #[test]
    #[cfg(feature = "swedish")]
    fn language_switching_month() {
        use crate::traits::WithLanguage;
        use crate::language::{Language, Swedish};

        let january_english = January::default();
        assert_eq!(format!("{}", january_english), "January");

        let january_swedish = january_english.with_language(Language::Swedish(Swedish::default()));
        assert_eq!(format!("{}", january_swedish), "Januari");
    }

    #[test]
    #[cfg(feature = "swedish")]
    fn language_switching_relative() {
        use crate::traits::WithLanguage;
        use crate::language::{Language, Swedish};

        let today_english = Today::default();
        assert_eq!(format!("{}", today_english), "Today");

        let today_swedish = today_english.with_language(Language::Swedish(Swedish::default()));
        assert_eq!(format!("{}", today_swedish), "Idag");
    }

    #[test]
    fn this_week_conversion() {
        let tuesday = base_time(); // July 29th, 2025

        let this_week = Relative::this_week();

        // This week should end on Sunday midnight (which is the next Monday)
        let max = this_week.to_chrono_max(tuesday);

        // Should be the upcoming Sunday at midnight (which shows as Monday)
        assert_eq!(max.weekday(), chrono::Weekday::Mon);
    }

    #[test]
    fn next_week_conversion() {
        let tuesday = base_time();

        let next_week = Relative::next_week();

        // Next week should end a week after this week ends
        let max = next_week.to_chrono_max(tuesday);

        // Should be Monday, 7 days after the end of this week
        assert_eq!(max.weekday(), chrono::Weekday::Mon);

        let this_week_max = Relative::this_week().to_chrono_max(tuesday);
        assert_eq!(max, this_week_max.checked_add_days(Days::new(7)).unwrap());
    }

    #[test]
    fn this_month_conversion() {
        let tuesday = base_time(); // July 29th, 2025

        let this_month = Relative::this_month();

        // This month should end at midnight on August 1st
        let max = this_month.to_chrono_max(tuesday);
        assert_eq!(max.month(), 8);
        assert_eq!(max.day(), 1);
        assert_eq!(max.year(), 2025);
    }

    #[test]
    fn month_year_boundary() {
        let dec_15 = DateTime::parse_from_rfc3339("2025-12-15T10:00:00-00:00")
            .unwrap()
            .to_utc();

        // December should wrap to January next year
        let december = Month::december();
        let max = december.to_chrono_max(dec_15, false);

        assert_eq!(max.year(), 2026);
        assert_eq!(max.month(), 1);
        assert_eq!(max.day(), 1);
    }

    #[test]
    fn month_january_in_december() {
        let dec_15 = DateTime::parse_from_rfc3339("2025-12-15T10:00:00-00:00")
            .unwrap()
            .to_utc();

        // January should be next year when we're in December
        let january = Month::january();
        let max = january.to_chrono_max(dec_15, false);

        assert_eq!(max.year(), 2026);
        assert_eq!(max.month(), 2);
        assert_eq!(max.day(), 1);
    }

    #[test]
    fn weekday_year_boundary() {
        // Test on December 31, 2025 (Wednesday)
        let wed_dec_31 = DateTime::parse_from_rfc3339("2025-12-31T10:00:00-00:00")
            .unwrap()
            .to_utc();

        // Monday should be next year
        let monday = Weekday::monday();
        let max = monday.to_chrono_max(wed_dec_31, false);

        assert_eq!(max.year(), 2026);
        assert_eq!(max.month(), 1);
    }

    #[test]
    fn from_max_chrono_preserves_language() {
        let tuesday = base_time();
        let midnight_tomorrow = tuesday
            .checked_add_days(Days::new(1))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        #[cfg(feature = "swedish")]
        {
            use crate::language::{Language, Swedish};

            let time = Time::from_max_chrono(
                midnight_tomorrow,
                Some(tuesday),
                Language::Swedish(Swedish::default()),
            );

            if let Time::Relative(Relative::Today(today)) = time {
                assert_eq!(format!("{}", today), "Idag");
            } else {
                panic!("Expected Today variant");
            }
        }

        #[cfg(not(feature = "swedish"))]
        {
            let time = Time::from_max_chrono(midnight_tomorrow, Some(tuesday), Language::default());

            if let Time::Relative(Relative::Today(today)) = time {
                assert_eq!(format!("{}", today), "Today");
            } else {
                panic!("Expected Today variant");
            }
        }
    }
}
