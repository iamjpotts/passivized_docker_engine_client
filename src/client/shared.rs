use std::io::Cursor;

use crate::errors::DecUseError;
use crate::imp::content_type;
use crate::model::StreamLine;
use crate::imp::http_proxy::DockerEngineHttpResponse;

pub(super) fn parse_container_log(response: DockerEngineHttpResponse) -> Result<Vec<StreamLine>, DecUseError> {
    let bytes = response
        .assume_content_type(content_type::STREAM)?
        .body
        .to_vec();

    let cursor = Cursor::new(bytes);
    let lines = StreamLine::read_all(cursor)?;

    Ok(lines)
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