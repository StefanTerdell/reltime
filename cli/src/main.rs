use chrono::{DateTime, Utc};
use clap::{Args, Parser};
use reltime::{
    Time,
    exact::{ExactDate, ExactDateTime, ExactTime},
    month::Month,
    relative::Relative,
    weekday::Weekday,
};
use schemars::schema_for;

#[derive(Debug, Clone, Parser)]
pub enum Value {
    Now,
    Today,
    Date {
        #[clap(long)]
        year: Option<i16>,
        month: u8,
        day: u8,
    },
    Time {
        hour: u8,
        minute: u8,
        second: Option<u8>,
    },
    DateTime {
        #[clap(long)]
        year: Option<i16>,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: Option<u8>,
    },
    Tomorrow,
    ThisWeek,
    NextWeek,
    ThisMonth,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Parse {
        value: String,
    },
}

impl TryFrom<Value> for Time {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(match value {
            Value::Now => Self::DateTime(Utc::now()),
            Value::Date { year, month, day } => {
                Self::Relative(Relative::Date(ExactDate::new(year, month, day)))
            }
            Value::Time {
                hour,
                minute,
                second,
            } => Self::Relative(Relative::Time(ExactTime::new(hour, minute, second))),
            Value::DateTime {
                year,
                month,
                day,
                hour,
                minute,
                second,
            } => Self::Relative(Relative::DateTime(ExactDateTime::new(
                ExactDate::new(year, month, day),
                ExactTime::new(hour, minute, second),
            ))),
            Value::Today => Self::Relative(Relative::today()),
            Value::Tomorrow => Self::Relative(Relative::tomorrow()),
            Value::ThisWeek => Self::Relative(Relative::this_week()),
            Value::NextWeek => Self::Relative(Relative::next_week()),
            Value::ThisMonth => Self::Relative(Relative::this_month()),
            Value::Monday => Self::Weekday(Weekday::monday()),
            Value::Tuesday => Self::Weekday(Weekday::tuesday()),
            Value::Wednesday => Self::Weekday(Weekday::wednesday()),
            Value::Thursday => Self::Weekday(Weekday::thursday()),
            Value::Friday => Self::Weekday(Weekday::friday()),
            Value::Saturday => Self::Weekday(Weekday::saturday()),
            Value::Sunday => Self::Weekday(Weekday::sunday()),
            Value::January => Self::Month(Month::january()),
            Value::February => Self::Month(Month::february()),
            Value::March => Self::Month(Month::march()),
            Value::April => Self::Month(Month::april()),
            Value::May => Self::Month(Month::may()),
            Value::June => Self::Month(Month::june()),
            Value::July => Self::Month(Month::july()),
            Value::August => Self::Month(Month::august()),
            Value::September => Self::Month(Month::september()),
            Value::October => Self::Month(Month::october()),
            Value::November => Self::Month(Month::november()),
            Value::December => Self::Month(Month::december()),
            Value::Parse { value } => serde_json::from_str(&format!("\"{}\"", value))?,
        })
    }
}

#[derive(Debug, Clone, Args)]
pub struct TimeArgs {
    #[clap(long, short)]
    relative_to: Option<DateTime<Utc>>,
    #[command(subcommand)]
    value: Value,
}

#[derive(Debug, Clone, Parser)]
pub enum Cli {
    Min(TimeArgs),
    Max(TimeArgs),
    Schema,
}

fn main() -> Result<(), serde_json::Error> {
    match Cli::parse() {
        Cli::Min(TimeArgs { relative_to, value }) => {
            let time = Time::try_from(value)?.to_chrono_min(relative_to.unwrap_or(Utc::now()));
            let json = serde_json::to_string_pretty(&time)?;
            println!("{json}");
        }
        Cli::Max(TimeArgs { value, relative_to }) => {
            let time = Time::try_from(value)?.to_chrono_max(relative_to.unwrap_or(Utc::now()));
            let json = serde_json::to_string_pretty(&time)?;
            println!("{json}");
        }
        Cli::Schema => {
            let schema = schema_for!(Time);
            let json = serde_json::to_string_pretty(&schema)?;
            println!("{json}");
        }
    };

    Ok(())
}
