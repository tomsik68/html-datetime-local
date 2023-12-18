use crate::*;
use proptest::{prop_assert_eq, proptest};
#[test]
fn parse_valid_datetime() {
    let datetime_str = "2023-12-18T12:34:56";
    let expected_datetime = Datetime {
        date: YearMonthDay {
            year: Year(2023),
            month: Month(12),
            day: Day(18),
        },
        time: HourMinuteSecond {
            hour: Hour(12),
            minute: Minute(34),
            second: Second(56.0),
        },
    };

    let parsed_datetime: Datetime = datetime_str
        .parse()
        .expect("Failed to parse valid Datetime");

    assert_eq!(parsed_datetime, expected_datetime);
}

#[test]
fn parse_invalid_datetime_missing_time() {
    let datetime_str = "2023-12-18";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DateTimeParseError { .. }));
}

#[test]
fn parse_invalid_datetime_invalid_components() {
    let datetime_str = "2023-13-45T25:70:80";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
}

#[test]
fn parse_invalid_datetime_invalid_year() {
    let datetime_str = "anno_domini-12-01T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError {
            component: Component::Year,
            found,
            kind: DateTimeParseErrorKind::InvalidNumber(_),
        } if found == "anno_domini"
    ));
}

#[test]
fn parse_invalid_datetime_invalid_month() {
    let datetime_str = "2023-15-01T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError {
            component: Component::Month,
            found,
            kind: DateTimeParseErrorKind::OutOfRange { .. },
        } if found == "15"
    ));
}

#[test]
fn parse_invalid_datetime_february_29_nonleap() {
    let datetime_str = "2023-02-29T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError {
            component: Component::Day,
            found,
            kind: DateTimeParseErrorKind::OutOfRange { .. },
        } if found == "29"
    ));
}

#[test]
fn parse_valid_datetime_february_29_leap() {
    let datetime_str = "2004-02-29T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_ok());
}

proptest! {

    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _: Result<Datetime, _> = s.parse();
    }

    #[test]
    fn parses_date_back_to_original_with_second(y in 0i32..10000,
                                    m in 1u8..=12, d in 1u8..=28, h in 0u8..=23, min in 0u8..=59, sec in 0f32..=59.9f32) {
        let s = format!("{y}-{m}-{d}T{h}:{min}:{sec}");
        let original = Datetime {
            date: YearMonthDay::from_components(y.try_into().unwrap(), m.try_into().unwrap(), d.try_into().unwrap()).unwrap(),
            time: HourMinuteSecond {
                hour: h.try_into().unwrap(),
                minute: min.try_into().unwrap(),
                second: sec.try_into().unwrap(),
            }
        };
        let result: Result<Datetime, _> = s.parse();
        let dt = result.unwrap();
        prop_assert_eq!(original, dt);
    }

    #[test]
    fn parses_date_back_to_original_without_second(y in 0i32..10000,
                                    m in 1u8..=12, d in 1u8..=28, h in 0u8..=23, min in 0u8..=59) {
        let original = Datetime {
            date: YearMonthDay::from_components(y.try_into().unwrap(), m.try_into().unwrap(), d.try_into().unwrap()).unwrap(),
            time: HourMinuteSecond {
                hour: h.try_into().unwrap(),
                minute: min.try_into().unwrap(),
                second: 0f32.try_into().unwrap(),
            }
        };
        let s = format!("{y}-{m}-{d}T{h}:{min}");
        let result: Result<Datetime, _> = s.parse();
        let dt = result.unwrap();
        prop_assert_eq!(original, dt);
    }
}
