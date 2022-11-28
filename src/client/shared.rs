use std::io::Cursor;
use time::format_description::well_known::Iso8601;

use time::OffsetDateTime;

use crate::errors::DecUseError;
use crate::imp::content_type;
use crate::model::StreamLine;
use crate::imp::http_proxy::DockerEngineHttpResponse;

/// Parse output of https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerLogs
pub(super) fn parse_container_log(response: DockerEngineHttpResponse) -> Result<Vec<StreamLine>, DecUseError> {
    let bytes = response
        .assume_content_type(content_type::STREAM)?
        .body
        .to_vec();

    let cursor = Cursor::new(bytes);
    let lines = StreamLine::read_all(cursor)?;

    Ok(lines)
}

/// Parse a time in the format 2022-11-28T00:34:45.107901180Z
fn parse_log_container_timestamp(text: &str) -> Result<OffsetDateTime, String> {
    OffsetDateTime::parse(text, &Iso8601::DEFAULT)
        .map_err(|e| format!("Invalid timestamp '{}': {}", text, e))
}

pub(crate) fn split_log_container_timestamp(line: &'_ str) -> Result<TimestampedLogLine<'_>, String> {
    const TIMESTAMP_LENGTH: usize = 30;

    if line.len() >= TIMESTAMP_LENGTH {
        let (ts_text, text) = line.split_at(TIMESTAMP_LENGTH);

        let ts = parse_log_container_timestamp(ts_text)?;

        Ok(
            TimestampedLogLine {
                timestamp: ts,
                text: if text.is_empty() { text } else { &text[1..] }
            }
        )
    }
    else {
        Err(format!("Line does not contain a timestamp prefix: {}", line))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TimestampedLogLine<'t> {
    pub(crate) timestamp: OffsetDateTime,
    pub(crate) text: &'t str
}

#[cfg(test)]
mod test_parse_container_log {
    use hyper::body::Bytes;
    use crate::errors::DecUseError;
    use crate::imp::content_type::STREAM;
    use crate::imp::http_proxy::DockerEngineHttpResponse;
    use crate::model::StreamKind;
    use super::parse_container_log;

    #[test]
    fn parses() {
        let response = DockerEngineHttpResponse {
            request_uri: Default::default(),
            status: Default::default(),
            content_type: Some(STREAM.into()),
            body: Bytes::from(&b"\x01\x00\x00\x00\x00\x00\x00\x02Hi"[..]),
        };

        let actual = parse_container_log(response)
            .unwrap();

        assert_eq!(1, actual.len());

        let line0 = actual.get(0)
            .unwrap();

        assert_eq!(StreamKind::StdOut, line0.kind);
        assert_eq!("Hi", line0.text);
    }

    #[test]
    fn wrong_content_type() {
        const WRONG: &str = "x-wrong/x-wrong";

        let response = DockerEngineHttpResponse {
            request_uri: Default::default(),
            status: Default::default(),
            content_type: Some(WRONG.into()),
            body: Default::default()
        };

        let actual = parse_container_log(response)
            .unwrap_err();

        if let DecUseError::UnexpectedResponseContentType { actual: Some(content_type), .. } = actual {
            assert_eq!(WRONG, content_type);
        }
        else {
            panic!("Unexpected error: {}", actual);
        }
    }
}

#[cfg(test)]
mod test_parse_container_log_timestamp {
    use time::{Date, Month, PrimitiveDateTime, Time};
    use crate::client::shared::parse_log_container_timestamp;

    #[test]
    fn parses() {
        let input = "2022-11-28T00:34:45.107901180Z";

        let actual = parse_log_container_timestamp(input)
            .unwrap();

        assert_eq!(
            PrimitiveDateTime::new(
                Date::from_calendar_date(2022, Month::November, 28)
                    .unwrap(),
                Time::from_hms_nano(0, 34, 45, 107901180)
                    .unwrap()
            )
                .assume_utc(),
            actual
        )
    }

    #[test]
    fn when_empty() {
        let actual = parse_log_container_timestamp("foo")
            .unwrap_err();

        assert!(actual.to_string().contains("foo"));
        assert!(actual.to_string().contains("could not be parsed"))
    }
}

#[cfg(test)]
mod test_split_container_log_timestamp_line {
    use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};
    use crate::client::shared::split_log_container_timestamp;

    fn expected_timestamp() -> OffsetDateTime {
        PrimitiveDateTime::new(
            Date::from_calendar_date(2022, Month::November, 28)
                .unwrap(),
            Time::from_hms_nano(0, 34, 45, 107901180)
                .unwrap()
        )
            .assume_utc()
    }

    #[test]
    fn does_not_split() {
        let input = "musk bought twitter";

        let actual = split_log_container_timestamp(input)
            .unwrap_err();

        assert_eq!(
            "Line does not contain a timestamp prefix: musk bought twitter",
            actual
        );
    }

    #[test]
    fn splits() {
        let input = "2022-11-28T00:34:45.107901180Z musk bought twitter";

        let actual = split_log_container_timestamp(input)
            .unwrap();

        assert_eq!(
            expected_timestamp(),
            actual.timestamp
        );

        assert_eq!(
            "musk bought twitter",
            actual.text
        )
    }

    #[test]
    fn splits_bare() {
        let input = "2022-11-28T00:34:45.107901180Z";

        let actual = split_log_container_timestamp(input)
            .unwrap();

        assert_eq!(
            expected_timestamp(),
            actual.timestamp
        );

        assert_eq!(
            "",
            actual.text
        )
    }

    #[test]
    fn splits_trailing_separator() {
        let input = "2022-11-28T00:34:45.107901180Z ";

        let actual = split_log_container_timestamp(input)
            .unwrap();

        assert_eq!(
            expected_timestamp(),
            actual.timestamp
        );

        assert_eq!(
            "",
            actual.text
        )
    }
}