//! # html-datetime-local
//!
//! [![GitHub license](https://img.shields.io/github/license/tomsik68/html-datetime-local?style=for-the-badge)](https://github.com/tomsik68/html-datetime-local/blob/master/LICENSE)
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/tomsik68/html-datetime-local/rust.yml?branch=master&style=for-the-badge)](https://github.com/tomsik68/html-datetime-local/actions/workflows/rust.yml)
//! [![Crates.io](https://img.shields.io/crates/v/html-datetime-local?style=for-the-badge)](https://crates.io/crates/html-datetime-local)
//! [![Crates.io (latest)](https://img.shields.io/crates/dv/html-datetime-local?style=for-the-badge)](https://crates.io/crates/html-datetime-local)
//!
//! ## Overview
//!
//! `html-datetime-local` is a Rust library for parsing local date and time strings based on the [WHATWG HTML Living Standard](https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#local-dates-and-times).
//!
//! This may be helpful for server-side code that deals with values from `<input type="datetime-local" />`.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! html-datetime-local = "0.1"
//! ```
//!
//! Then, in your Rust code:
//! ```rust
//! use html_datetime_local::Datetime;
//! use std::str::FromStr;
//!
//! let input = "2023-12-31T23:59:59";
//! match Datetime::from_str(input) {
//!     Ok(datetime) => println!("Parsed datetime: {:?}", datetime),
//!     Err(err) => eprintln!("Error parsing datetime: {}", err),
//! }
//! ```
//!
//! # Contributing
//!
//! Pull requests and bug reports are welcome! If you have any questions or suggestions, feel free to open an issue.
//!
//! # License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.

use anyhow::Error;
use std::convert::TryFrom;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Datetime {
    pub date: YearMonthDay,
    pub time: HourMinuteSecond,
}

impl FromStr for Datetime {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('T');

        let date = YearMonthDay::from_str(parts.next().ok_or_else(|| DateTimeParseError {
            component: Component::Date,
            found: "".to_string(),
            kind: DateTimeParseErrorKind::ValueMissing,
        })?)?;

        let time = HourMinuteSecond::from_str(parts.next().ok_or_else(|| DateTimeParseError {
            component: Component::Time,
            found: "".to_string(),
            kind: DateTimeParseErrorKind::ValueMissing,
        })?)?;

        Ok(Datetime { date, time })
    }
}

#[derive(Debug, Error)]
#[error("Failed to parse {component}'s value `{found}`: {kind}")]
pub struct DateTimeParseError {
    component: Component,
    found: String,
    kind: DateTimeParseErrorKind,
}

#[derive(Debug, Error)]
pub enum DateTimeParseErrorKind {
    #[error(transparent)]
    InvalidNumber(Error),
    #[error("The value is missing")]
    ValueMissing,
    #[error("The value must be at least {min} and at most {max}")]
    OutOfRange { min: i32, max: i32 },
}

#[derive(Debug, PartialEq, Clone, strum::Display)]
pub enum Component {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,

    Date,
    Time,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Year(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Month(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Day(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Hour(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Minute(u8);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Second(f32);

#[derive(Debug, PartialEq, Clone)]
pub struct YearMonthDay {
    year: Year,
    month: Month,
    day: Day,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HourMinuteSecond {
    hour: Hour,
    minute: Minute,
    second: Second,
}

macro_rules! impl_parse_numeric {
    ($component:tt, $inner:ty, $min:expr, $max:expr) => {
        impl TryFrom<$inner> for $component {
            type Error = DateTimeParseError;

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                if !(($min as $inner)..=($max as $inner)).contains(&value) {
                    return Err(DateTimeParseError {
                        component: Component::$component,
                        found: value.to_string(),
                        kind: DateTimeParseErrorKind::OutOfRange {
                            min: $min,
                            max: ($max - 1),
                        },
                    });
                }

                Ok(Self(value))
            }
        }

        impl FromStr for $component {
            type Err = DateTimeParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let inner =
                    <$inner as FromStr>::from_str(value).map_err(|source| DateTimeParseError {
                        component: Component::$component,
                        found: value.to_string(),
                        kind: DateTimeParseErrorKind::InvalidNumber(source.into()),
                    })?;

                Self::try_from(inner)
            }
        }
    };
}

impl_parse_numeric!(Year, i32, i32::MIN, i32::MAX);
impl_parse_numeric!(Month, u8, 1, 13);
impl_parse_numeric!(Day, u8, 1, 32);
impl_parse_numeric!(Hour, u8, 0, 24);
impl_parse_numeric!(Minute, u8, 0, 60);
impl_parse_numeric!(Second, f32, 0, 60);

impl FromStr for YearMonthDay {
    type Err = DateTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split('-').collect();

        let year = parts.first().ok_or_else(|| DateTimeParseError {
            found: "".to_string(),
            component: Component::Year,
            kind: DateTimeParseErrorKind::ValueMissing,
        })?;
        let month = parts.get(1).ok_or_else(|| DateTimeParseError {
            found: "".to_string(),
            component: Component::Month,
            kind: DateTimeParseErrorKind::ValueMissing,
        })?;
        let day = parts.get(2).ok_or_else(|| DateTimeParseError {
            found: "".to_string(),
            component: Component::Day,
            kind: DateTimeParseErrorKind::ValueMissing,
        })?;

        let year = Year::from_str(year)?;
        let month = Month::from_str(month)?;
        let day = Day::from_str(day)?;

        Self::from_components(year, month, day)
    }
}

impl YearMonthDay {
    pub fn from_components(year: Year, month: Month, day: Day) -> Result<Self, DateTimeParseError> {
        if !is_valid_day(year, month, day) {
            return Err(DateTimeParseError {
                kind: DateTimeParseErrorKind::OutOfRange {
                    min: 1,
                    max: day_in_month(year, month) as i32,
                },
                found: day.0.to_string(),
                component: Component::Day,
            });
        }

        Ok(YearMonthDay { year, month, day })
    }
}

// Helper function to check if the given day is valid for the given year and month.
fn is_valid_day(year: Year, month: Month, day: Day) -> bool {
    day.0 <= day_in_month(year, month)
}

// Helper function to determine the number of days in a month.
fn day_in_month(year: Year, month: Month) -> u8 {
    match month.0 {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year.0) => 29,
        2 => 28,
        _ => unreachable!("The Month type guards against values that aren't in range (1..=12)"),
    }
}

// Helper function to check if a year is a leap year.
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

impl FromStr for HourMinuteSecond {
    type Err = DateTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(':').collect();

        let hour = parts.first().ok_or_else(|| DateTimeParseError {
            component: Component::Hour,
            found: value.to_string(),
            kind: DateTimeParseErrorKind::ValueMissing,
        })?;
        let minute = parts.get(1).ok_or_else(|| DateTimeParseError {
            component: Component::Minute,
            found: value.to_string(),
            kind: DateTimeParseErrorKind::ValueMissing,
        })?;

        let second = parts.get(2).unwrap_or(&"0");

        Ok(HourMinuteSecond {
            hour: Hour::from_str(hour)?,
            minute: Minute::from_str(minute)?,
            second: Second::from_str(second)?,
        })
    }
}
