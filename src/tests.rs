use crate::*;
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
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError::TimeComponentError { .. }
    ));
}

#[test]
fn parse_invalid_datetime_invalid_components() {
    let datetime_str = "2023-13-45T25:70:80";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError::InvalidMonth { .. }
    ));
}

#[test]
fn parse_invalid_datetime_invalid_month() {
    let datetime_str = "2023-13-01T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError::InvalidMonth { .. }
    ));
}

#[test]
fn parse_invalid_datetime_invalid_year() {
    let datetime_str = "anno_domini-12-01T01:01:01";
    let result: Result<Datetime, _> = datetime_str.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DateTimeParseError::InvalidYear { .. }
    ));
}
