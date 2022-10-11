use std::str::FromStr;
use url::{ParseError, Url};

/*
    A fluent style URL builder that escapes paths and query values.

    Example:
        let server = UrlBuilder::from_str("http://server")?;

        let url = server
            .join("images")?
            .join(image_id)?
            .join("push")?
            .query()
            .append("tag", tag)
            .to_string();

    Docker Engine IDs (such as container names or volume names) are not allowed
    to have any characters that would require path or URL encoding. However, to
    prevent malicious value injection, we will encode them anyways.

    Note that forward slashes ARE allowed. For example:

        ...given image ID "private-registry.com:8443/repo" and tag "latest"
        ...and request path /images/{name or id}/push?tag={tag}

        ...the path expands to:
        /images/private-registry.com:8443/repo/push?tag=latest

        ...which is fine, even though "push" is one level deeper.

    Differences from url crate:
        * Join does not require a trailing slash
        * Join does not resolve a leading slash
        * Query parameters can be added with a fluent style
 */

#[derive(Clone, Debug)]
pub(crate) struct UrlBuilder {
    value: Url
}

impl UrlBuilder {

    pub fn from_str(value: &str) -> Result<Self, ParseError> {
        Ok(Self {
            value: Url::from_str(value)?
        })
    }

    pub fn join(&self, path: &str) -> Result<Self, ParseError> {
        let s = self.value.to_string();

        if s.ends_with('/') {
            Ok(Self {
                value: self.value.join(path)?
            })
        }
        else {
            Ok(Self {
                value: Url::from_str(&format!("{}/", s))?
                    .join(path)?
            })
        }
    }

    pub fn query(&self) -> UrlQueryBuilder {
        UrlQueryBuilder {
            value: self.value.clone()
        }
    }
}

impl ToString for UrlBuilder {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UrlQueryBuilder {
    value: Url
}

impl UrlQueryBuilder {
    pub fn append<V: ToString>(mut self, name: &str, value: V) -> Self {
        self.value.query_pairs_mut().append_pair(name, &value.to_string());
        self
    }

    pub fn option<V: ToString>(self, name: &str, value: Option<V>) -> Self {
        match value {
            None => self,
            Some(v) => self.append(name, v)
        }
    }
}

impl ToString for UrlQueryBuilder {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
pub mod test_url_builder {
    use url::ParseError;
    use crate::imp::url::UrlBuilder;

    #[test]
    pub fn appends_path_to_base_with_trailing_slash() {
        let base = UrlBuilder::from_str("http://a/")
            .unwrap();

        let sub = base.join("b")
            .unwrap();

        assert_eq!("http://a/b", &sub.to_string());
    }

    #[test]
    pub fn appends_path_to_base_without_trailing_slash() {
        let base = UrlBuilder::from_str("http://a")
            .unwrap();

        let sub = base.join("b")
            .unwrap();

        assert_eq!("http://a/b", &sub.to_string());
    }

    #[test]
    pub fn appends_path_to_path_with_trailing_slash() {
        let base = UrlBuilder::from_str("http://a/b/")
            .unwrap();

        let sub = base.join("c")
            .unwrap();

        assert_eq!("http://a/b/c", &sub.to_string());
    }

    #[test]
    pub fn appends_path_to_path_without_trailing_slash() {
        let base = UrlBuilder::from_str("http://a/b")
            .unwrap();

        let sub = base.join("c")
            .unwrap();

        assert_eq!("http://a/b/c", &sub.to_string());
    }

    #[test]
    pub fn builds_path_and_query() -> Result<(), ParseError> {
        let actual = UrlBuilder::from_str("https://testuri.org:123")?
            .join("images")?
            .join("foo/bar")?
            .join("push")?
            .query()
            .append("repo", "qux")
            .append("tag", "baz")
            .to_string();

        assert_eq!("https://testuri.org:123/images/foo/bar/push?repo=qux&tag=baz", &actual);

        Ok(())
    }

    #[test]
    pub fn builds_path_and_optional_query() -> Result<(), ParseError> {
        let actual = UrlBuilder::from_str("unix://some-fd:0")?
            .join("path")?
            .query()
            .append("one", 1)
            .option("two", Some(2))
            .option("three", Option::<usize>::None)
            .append("maybe", true)
            .to_string();

        assert_eq!("unix://some-fd:0/path?one=1&two=2&maybe=true", &actual);

        Ok(())
    }

    #[test]
    pub fn encodes_path() {
        let sub = UrlBuilder::from_str("http://a")
            .unwrap()
            .join("Hello, World")
            .unwrap();

        assert_eq!("http://a/Hello,%20World", &sub.to_string());
    }

    #[test]
    pub fn encodes_query_value() {
        let sub = UrlBuilder::from_str("http://a")
            .unwrap()
            .query()
            .append("greeting", "Hello, world.");

        assert_eq!("http://a/?greeting=Hello%2C+world.", &sub.to_string());
    }

    #[test]
    pub fn errors_from_empty_string() {
        let error = UrlBuilder::from_str("")
            .unwrap_err();

        match error {
            ParseError::RelativeUrlWithoutBase => {}
            _ => panic!("Unexpected error: {:?}", error)
        }
    }

    #[test]
    pub fn errors_from_invalid_url() {
        let error = UrlBuilder::from_str("https:://a")
            .unwrap_err();

        match error {
            ParseError::EmptyHost => {}
            _ => panic!("Unexpected error: {:?}", error)
        }
    }

}
