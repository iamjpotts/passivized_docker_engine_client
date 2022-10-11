use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Default)]
pub struct ListContainersRequest {
    pub all: Option<bool>,
    pub limit: Option<usize>,
    pub size: Option<bool>,
    pub filters: Filters,
}

impl ListContainersRequest {

    pub fn all(mut self, v: bool) -> Self {
        self.all = Some(v);
        self
    }

    pub fn limit(mut self, v: usize) -> Self {
        self.limit = Some(v);
        self
    }

    pub fn size(mut self, v: bool) -> Self {
        self.size = Some(v);
        self
    }

    pub fn filters(mut self, v: Filters) -> Self {
        self.filters = v;
        self
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Default, Debug, Serialize)]
pub struct Filters {

    // Note that "label" in the serde rename is intentionally lowercase, not Title case.
    #[serde(rename = "label", serialize_with = "sz_labels")]
    labels: HashMap<String, Option<String>>

}

impl Filters {

    /// Return true if no filters are set.
    pub(crate) fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Add a filter that requires a label to be present. The value of the label does not matter.
    pub fn label_present<K: Into<String>>(mut self, k: K) -> Self {
        self.labels.insert(k.into(), None);
        self
    }

    /// Add a filter that requires a label to be present and match a specific value.
    pub fn label_value<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> Self {
        self.labels.insert(k.into(), Some(v.into()));
        self
    }

}

// Filters gets serialized into JSON struct that is input for a URL query parameter.
fn sz_labels<SZ>(labels: &HashMap<String, Option<String>>, serializer: SZ) -> Result<SZ::Ok, SZ::Error>
    where SZ: Serializer
{
    let mut sequence_sz = serializer.serialize_seq(Some(labels.len()))?;

    for (k, ov) in labels {
        match ov {
            Some(v) => {
                sequence_sz.serialize_element(&format!("{}={}", k, v))?;
            },
            None => {
                sequence_sz.serialize_element(k)?;
            }
        }
    }

    sequence_sz.end()
}

#[cfg(test)]
pub mod test_serialize_filters {
    use super::Filters;

    #[test]
    pub fn empty() {
        let filters = Filters::default();

        // Don't care what it serializes to b/c it will get omitted
        assert!(filters.is_empty());
    }

    #[test]
    pub fn label_only() {
        let filters = Filters::default()
            .label_present("foo");

        let actual = serde_json::to_string(&filters)
            .unwrap();

        assert_eq!("{\"label\":[\"foo\"]}".to_string(), actual);
    }

    #[test]
    pub fn label_with_value() {
        let filters = Filters::default()
            .label_value("foo", "bar");

        let actual = serde_json::to_string(&filters)
            .unwrap();

        assert_eq!("{\"label\":[\"foo=bar\"]}".to_string(), actual);
    }
}