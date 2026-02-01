use chrono::{DateTime, Months, NaiveTime, Utc};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    language::Language,
    month::{
        April, August, December, February, January, July, June, March, May, Month, November,
        October, September,
    },
    relative::{Relative, ThisMonth, ThisWeek, Today, Tomorrow},
    traits::FromLanguage,
    weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday, Weekday},
};

pub mod language;
pub mod month;
pub mod relative;
pub mod traits;
pub mod weekday;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Display)]
#[serde(untagged)]
pub enum Time {
    Relative(Relative),
    Weekday(Weekday),
    Month(Month),
    DateTime(DateTime<Utc>),
}

impl Time {
    pub fn to_chrono_min_now(self) -> DateTime<Utc> {
        self.to_chrono_min(Utc::now())
    }

    pub fn to_chrono_min(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Time::Relative(relative) => relative.to_chrono_min(relative_to),
            Time::Weekday(weekday) => weekday.to_chrono_min(relative_to, true),
            Time::Month(month) => month
                .next_midnight(relative_to, true)
                .checked_sub_months(Months::new(1))
                .unwrap(),
            Time::DateTime(date_time) => date_time.max(relative_to),
        }
    }

    pub fn to_chrono_max_now(self) -> DateTime<Utc> {
        self.to_chrono_max(Utc::now())
    }

    pub fn to_chrono_max(self, relative_to: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Time::Relative(relative) => relative.to_chrono_max(relative_to),
            Time::Weekday(weekday) => weekday.to_chrono_max(relative_to, true),
            Time::Month(month) => month.next_midnight(relative_to, true),
            Time::DateTime(date_time) => date_time,
        }
    }

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

            let this_week = Relative::ThisWeek(ThisWeek::from_language(language));
            if date_time == this_week.clone().to_chrono_max(now) {
                return Time::Relative(this_week);
            }

            let this_month = Relative::ThisMonth(ThisMonth::from_language(language));
            if date_time == this_month.clone().to_chrono_max(now) {
                return Time::Relative(this_month);
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
        }

        Time::DateTime(date_time)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Days};

    use super::*;

    /// Creates a DateTime\<Utc> for Tueday July 29th, 2025 at 10:30:05.
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

        let midnight_next_monday = tuesday
            .checked_add_days(Days::new(6))
            .unwrap()
            .with_time(NaiveTime::MIN)
            .unwrap();

        assert_eq!(midnight_next_monday.weekday(), chrono::Weekday::Mon,);

        let time = Time::from_max_chrono(midnight_next_monday, Some(tuesday), Language::default());

        println!(
            "sunday chrono max {}",
            Time::Weekday(Weekday::sunday()).to_chrono_max(tuesday)
        );

        assert_eq!(time, Time::Weekday(Weekday::sunday()));
    }
}
