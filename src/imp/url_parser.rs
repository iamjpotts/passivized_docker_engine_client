
pub(crate) fn is_http(uri: &str) -> bool {
    uri.starts_with("http://") || uri.starts_with("https://")
}

#[cfg(test)]
pub mod test_is_http {

    mod returns_false {
        use crate::imp::url_parser::is_http;

        #[test]
        pub fn when_empty() {
            assert!(!is_http(""))
        }

        #[test]
        pub fn when_tcp_uri() {
            assert!(!is_http("tcp://some-server:123"))
        }

        #[test]
        pub fn when_unix_uri() {
            assert!(!is_http("unix:///some/path"))
        }
    }

    mod returns_true {
        use crate::imp::url_parser::is_http;

        #[test]
        pub fn when_http() {
            assert!(is_http("http://foo"))
        }

        #[test]
        pub fn when_https() {
            assert!(is_http("https://foo"))
        }
    }
}