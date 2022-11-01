use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BuildImageResponseStreamItem {

    pub stream: Option<String>,

    #[serde(rename = "errorDetail")]
    pub error_detail: Option<BuildImageResponseStreamItemErrorDetail>,

    pub error: Option<String>,

}

impl BuildImageResponseStreamItem {

    pub fn has_error(&self) -> bool {
        self.error.is_some() || self.error_detail.is_some()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BuildImageResponseStreamItemErrorDetail {

    pub message: String,

}

#[cfg(test)]
mod test_build_image_response_stream_item {

    mod test_has_error {
        use crate::responses::{BuildImageResponseStreamItem, BuildImageResponseStreamItemErrorDetail};

        #[test]
        fn when_none() {
            let value = BuildImageResponseStreamItem {
                stream: Some("foo".into()),
                error: None,
                error_detail: None
            };

            assert!(!value.has_error());
        }

        #[test]
        fn when_error() {
            let value = BuildImageResponseStreamItem {
                stream: None,
                error: Some("bar".into()),
                error_detail: None
            };

            assert!(value.has_error());
        }

        #[test]
        fn when_error_detail() {
            let value = BuildImageResponseStreamItem {
                stream: None,
                error: None,
                error_detail: Some(BuildImageResponseStreamItemErrorDetail {
                    message: "qux".into()
                })
            };

            assert!(value.has_error());
        }
    }

}