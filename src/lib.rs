use anyhow::{Context, Error};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
struct Datetime {
    date: YearMonthDay,
    time: HourMinuteSecond,
}

impl FromStr for Datetime {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('T');

        let date = YearMonthDay::from_str(parts.next().ok_or(
            DateTimeParseError::DateComponentError {
                source: anyhow::anyhow!("The string does not contain a date component"),
                part: "".to_string(),
            },
        )?)?;
        let time = HourMinuteSecond::from_str(parts.next().ok_or(
            DateTimeParseError::TimeComponentError {
                source: anyhow::anyhow!("The string does not contain a time component"),
                part: "".to_string(),
            },
        )?)?;

        Ok(Datetime { date, time })
    }
}

#[derive(Debug, Error)]
enum DateTimeParseError {
    #[error("Failed to parse date component: {source}")]
    DateComponentError {
        #[source]
        source: Error,
        part: String,
    },
    #[error("Failed to parse time component: {source}")]
    TimeComponentError {
        #[source]
        source: Error,
        part: String,
    },
    #[error("Unexpected characters beyond the end of input.")]
    UnexpectedCharacters,
    #[error("Failed to parse year and month: {source}")]
    YearMonthError {
        #[source]
        source: Error,
        part: String,
    },
    #[error("Invalid day in the month. Found: {found}")]
    InvalidDay {
        found: String,
        #[source]
        source: Option<Error>,
    },
    #[error("Invalid hour in the time. Found: {found}")]
    InvalidHour {
        found: String,
        #[source]
        source: Option<Error>,
    },
    #[error("Invalid minute in the time. Found: {found}")]
    InvalidMinute {
        found: String,
        #[source]
        source: Option<Error>,
    },
    #[error("Invalid second in the time. Found: {found}")]
    InvalidSecond {
        found: String,
        #[source]
        source: Option<Error>,
    },
    #[error("Invalid month. Found: {found}")]
    InvalidMonth {
        found: String,
        #[source]
        source: Option<Error>,
    },
    #[error("Invalid year. Found: {found}")]
    InvalidYear {
        found: String,
        #[source]
        source: Option<Error>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Year(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Month(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Day(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Hour(u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Minute(u8);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Second(f32);

#[derive(Debug, PartialEq, Clone)]
struct YearMonthDay {
    year: Year,
    month: Month,
    day: Day,
}

#[derive(Debug, PartialEq, Clone)]
struct HourMinuteSecond {
    hour: Hour,
    minute: Minute,
    second: Second,
}

impl TryFrom<i32> for Year {
    type Error = DateTimeParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Year(value))
    }
}

impl TryFrom<u8> for Month {
    type Error = DateTimeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1..=12).contains(&value) {
            Ok(Month(value))
        } else {
            Err(DateTimeParseError::InvalidMonth {
                found: value.to_string(),
                source: None,
            })
        }
    }
}

impl TryFrom<u8> for Day {
    type Error = DateTimeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1..=31).contains(&value) {
            Ok(Day(value))
        } else {
            Err(DateTimeParseError::InvalidDay {
                found: value.to_string(),
                source: None,
            })
        }
    }
}

impl TryFrom<u8> for Hour {
    type Error = DateTimeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=23).contains(&value) {
            Ok(Hour(value))
        } else {
            Err(DateTimeParseError::InvalidHour {
                found: value.to_string(),
                source: None,
            })
        }
    }
}

impl TryFrom<u8> for Minute {
    type Error = DateTimeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=59).contains(&value) {
            Ok(Minute(value))
        } else {
            Err(DateTimeParseError::InvalidMinute {
                found: value.to_string(),
                source: None,
            })
        }
    }
}

impl TryFrom<f32> for Second {
    type Error = DateTimeParseError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if (0.0..60.0).contains(&value) {
            Ok(Second(value))
        } else {
            Err(DateTimeParseError::InvalidSecond {
                found: value.to_string(),
                source: None,
            })
        }
    }
}

impl FromStr for Year {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i32 = s
            .parse()
            .map_err(|source| DateTimeParseError::InvalidYear {
                found: String::from(s),
                source: Some(Error::from(source)),
            })?;

        Year::try_from(value)
    }
}

impl FromStr for Month {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s
            .parse()
            .map_err(|source| DateTimeParseError::InvalidMonth {
                found: String::from(s),
                source: Some(Error::from(source)),
            })?;

        Month::try_from(value)
    }
}

impl FromStr for Day {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s.parse().map_err(|source| DateTimeParseError::InvalidDay {
            found: String::from(s),
            source: Some(Error::from(source)),
        })?;

        Day::try_from(value)
    }
}

impl FromStr for Hour {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s
            .parse()
            .map_err(|source| DateTimeParseError::InvalidHour {
                found: String::from(s),
                source: Some(Error::from(source)),
            })?;

        Hour::try_from(value)
    }
}

impl FromStr for Minute {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s
            .parse()
            .map_err(|source| DateTimeParseError::InvalidMinute {
                found: String::from(s),
                source: Some(Error::from(source)),
            })?;

        Minute::try_from(value)
    }
}

impl FromStr for Second {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: f32 = s
            .parse()
            .map_err(|source| DateTimeParseError::InvalidSecond {
                found: String::from(s),
                source: Some(Error::from(source)),
            })?;

        Second::try_from(value)
    }
}

impl FromStr for YearMonthDay {
    type Err = DateTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split('-').collect();

        let year = parts
            .get(0)
            .ok_or_else(|| DateTimeParseError::YearMonthError {
                part: String::from(value),
                source: Error::msg("Invalid year"),
            })?;

        let month = parts
            .get(1)
            .ok_or_else(|| DateTimeParseError::InvalidMonth {
                found: String::from(value),
                source: None,
            })?;

        let day = parts.get(2).ok_or_else(|| DateTimeParseError::InvalidDay {
            found: String::from(value),
            source: None,
        })?;

        Ok(YearMonthDay {
            year: Year::from_str(year)?,
            month: Month::from_str(month)?,
            day: Day::from_str(day)?,
        })
    }
}

impl FromStr for HourMinuteSecond {
    type Err = DateTimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(':').collect();

        let hour = parts
            .get(0)
            .ok_or_else(|| DateTimeParseError::InvalidHour {
                found: String::from(value),
                source: None,
            })?;

        let minute = parts
            .get(1)
            .ok_or_else(|| DateTimeParseError::InvalidMinute {
                found: String::from(value),
                source: None,
            })?;

        let second = parts
            .get(2)
            .ok_or_else(|| DateTimeParseError::InvalidSecond {
                found: String::from(value),
                source: None,
            })?;

        Ok(HourMinuteSecond {
            hour: Hour::from_str(hour)?,
            minute: Minute::from_str(minute)?,
            second: Second::from_str(second)?,
        })
    }
}
