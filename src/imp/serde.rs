use std::collections::HashMap;
use std::iter::{FromIterator, zip};
use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeOwned, Error};
use serde_json::Value;
use crate::model::Unit;

pub(crate) fn dz_empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>
{
    let deserialized: Option<String> = Option::deserialize(deserializer)?;
    let result = deserialized.filter(|v| !v.is_empty());
    Ok(result)
}

/// Treat empty JSON map {} the same as being null or absent.
pub(crate) fn dz_empty_object_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: 'de + DeserializeOwned,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Parsed<U> {
        // At least the minimum required fields are present.
        AllRequiredFields(U),

        // Some required fields missing, or an empty json map, or a wrong type altogether, such as a string.
        Other(Value)
    }

    let result = match Option::<Parsed<T>>::deserialize(deserializer)? {
        None => None,
        Some(pa) => match pa {
            Parsed::AllRequiredFields(value) => Some(value),
            Parsed::Other(value) => {
                let wrong = match value {
                    Value::Object(other) => {
                        if other.is_empty() {
                            None
                        }
                        else {
                            Some(Value::Object(other))
                        }
                    },
                    other => Some(other)
                };

                match wrong {
                    None => None,
                    Some(other) => {
                        let message = match serde_json::from_value::<T>(other) {
                            Ok(_) => {
                                // This should never happen, but if it does, fallback to a reasonable error message.
                                format!("Invalid input for {}", std::any::type_name::<T>())
                            },
                            Err(problem) => {
                                format!("{problem} for {}", std::any::type_name::<T>())
                            }
                        };

                        return Err(Error::custom(message));
                    }
                }
            }
        }
    };

    Ok(result)
}

pub(crate) fn dz_hashmap<'de, D, V>(deserializer: D) -> Result<HashMap<String, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de>
{
    let deserialized = Option::deserialize(deserializer)?;
    Ok(deserialized.unwrap_or_default())
}

// Parse a JSON object, returning its fields as the entries of a hash map, but
// throwing away the fields and values of any nested object.
//
// Parses null as an empty hash map.
//
// Does not handle an absent field; that must be configured with a #[serde(default=)] annotation.
pub(crate) fn dz_hashmap_keys<'de, D, K>(deserializer: D) -> Result<HashMap<K, Unit>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Eq + core::hash::Hash + Clone
{
    let ohm: Option<HashMap<K, Option<Value>>> = Option::deserialize(deserializer)?;

    Ok(match ohm {
        None => {
            HashMap::new()
        }
        Some(hm) => {
            HashMap::from_iter(
                zip(
                    hm.keys().cloned(),
                    (0..).map(|_| Unit {})
                )
            )
        }
    })
}

/// Deserialize a hashmap of vectors where the values (which are vectors) can be null. A null is
/// deserialized to the default value, an empty vector.
pub(crate) fn dz_hashmap_of_nullable<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, Vec<V>>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Clone + Eq + core::hash::Hash,
        V: Deserialize<'de> + Clone
{
    let ohm: Option<HashMap<K, Option<Vec<V>>>> = Option::deserialize(deserializer)?;

    Ok(match ohm {
        None => {
            HashMap::new()
        }
        Some(hm) => {
            HashMap::from_iter(
                hm
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone().unwrap_or_default()))
            )
        }
    })
}

pub(crate) fn dz_vec<'de, D, V>(deserializer: D) -> Result<Vec<V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de>
{
    let deserialized = Option::deserialize(deserializer)?;
    Ok(deserialized.unwrap_or_default())
}

#[cfg(test)]
mod test_dz_empty_as_none {
    use serde::Deserialize;
    use super::dz_empty_as_none;

    #[derive(Debug, Deserialize)]
    struct HasFoo {
        #[serde(default, deserialize_with = "dz_empty_as_none")]
        pub foo: Option<String>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"foo\": \"\"
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"foo\": null
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"foo\": \"bar\"
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(Some("bar".into()), parsed.foo);
    }

}

#[cfg(test)]
mod test_dz_empty_object_as_none {
    use serde::Deserialize;
    use serde_json::error::Category;
    use super::dz_empty_object_as_none;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo {
        pub bar: String,
        pub zzz: Option<String>
    }

    #[derive(Debug, Deserialize)]
    struct HasFoo {
        #[serde(default, deserialize_with = "dz_empty_object_as_none")]
        pub foo: Option<Foo>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"foo\": {}
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"foo\": null
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(None, parsed.foo);
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"foo\": {
                \"bar\": \"qux\"
            }
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(Some(Foo { bar: "qux".into(), zzz: None }), parsed.foo);
    }

    #[test]
    fn when_missing_required_field() {
        // Provide the optional field but not the required field
        const TEXT: &str = "
        {
            \"foo\": {
                \"zzz\": \"abc\"
            }
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let failure = parse_result.unwrap_err();
        let failure_msg = format!("{failure}");

        assert_eq!(Category::Data, failure.classify());

        {
            let expected = "bar";
            assert!(failure_msg.contains(expected), "Should contain {}: {}", expected, failure_msg);
        }

        {
            let expected = std::any::type_name::<Foo>();
            assert!(failure_msg.contains(expected), "Should contain {}: {}", expected, failure_msg);
        }
    }

    #[test]
    fn when_wrong_type() {
        const TEXT: &str = "
        {
            \"foo\": \"ooga booga booga\"
        }";

        let parse_result: serde_json::Result<HasFoo> = serde_json::from_str(TEXT);
        let failure = parse_result.unwrap_err();
        let failure_msg = format!("{failure}");

        assert_eq!(Category::Data, failure.classify());

        {
            let expected = std::any::type_name::<Foo>();
            assert!(failure_msg.contains(expected), "Should contain {}: {}", expected, failure_msg);
        }
    }
}

#[cfg(test)]
mod test_dz_hashmap {
    use std::collections::HashMap;
    use serde::Deserialize;
    use super::dz_hashmap;

    #[derive(Debug, Deserialize)]
    struct HasMap {
        #[serde(default, deserialize_with = "dz_hashmap")]
        pub items: HashMap<String, String>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"items\": {}
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"items\": null
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"items\": {
                \"foo\": \"bar\",
                \"qux\": \"baz\"
            }
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            HashMap::from([("foo".into(), "bar".into()), ("qux".into(), "baz".into())]),
            parsed.items
        )
    }

    #[test]
    fn when_value_null() {
        const TEXT: &str = "
        {
            \"items\": {
                \"qux\": null
            }
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);

        let parsed = parse_result.unwrap_err();
        assert!(parsed.is_data());

        let message = format!("{:?}", parsed);
        assert!(message.contains("invalid type: null, expected a string"));
    }

}

#[cfg(test)]
mod test_dz_hashmap_keys {
    use std::collections::HashMap;
    use serde::Deserialize;
    use crate::model::Unit;
    use super::dz_hashmap_keys;

    #[derive(Debug, Deserialize)]
    struct HasEmptyMap {
        #[serde(default, deserialize_with = "dz_hashmap_keys")]
        pub items: HashMap<String, Unit>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasEmptyMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"items\": {}
        }";

        let parse_result: serde_json::Result<HasEmptyMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"items\": null
        }";

        let parse_result: serde_json::Result<HasEmptyMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"items\": {
                \"foo\": {},
                \"bar\": {}
            }
        }";

        let parse_result: serde_json::Result<HasEmptyMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(2, parsed.items.len())
    }

    #[test]
    fn when_value_null() {
        const TEXT: &str = "
        {
            \"items\": {
                \"qux\": null
            }
        }";

        let parse_result: serde_json::Result<HasEmptyMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(1, parsed.items.len())
    }
}

#[cfg(test)]
mod test_dz_hashmap_of_nullable {
    use std::collections::HashMap;
    use serde::Deserialize;
    use super::dz_hashmap_of_nullable;

    #[derive(Debug, Deserialize)]
    struct HasMap {
        #[serde(default, deserialize_with = "dz_hashmap_of_nullable")]
        pub items: HashMap<String, Vec<i32>>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"items\": {}
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"items\": null
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(HashMap::new(), parsed.items)
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"items\": {
                \"foo\": [123],
                \"qux\": [456]
            }
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            HashMap::from([
                ("foo".into(), vec![123]),
                ("qux".into(), vec![456])
            ]),
            parsed.items
        )
    }

    #[test]
    fn when_value_null() {
        const TEXT: &str = "
        {
            \"items\": {
                \"qux\": null
            }
        }";

        let parse_result: serde_json::Result<HasMap> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            HashMap::from([("qux".into(), Vec::new())]),
            parsed.items
        )
    }

}

#[cfg(test)]
mod test_dz_vec {
    use serde::Deserialize;
    use super::dz_vec;

    #[derive(Debug, Deserialize)]
    struct HasVec {
        #[serde(default, deserialize_with = "dz_vec")]
        pub items: Vec<i32>
    }

    #[derive(Debug, Deserialize)]
    struct HasStrVec {
        #[serde(default, deserialize_with = "dz_vec")]
        pub items: Vec<String>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<i32> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"items\": []
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<i32> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"items\": null
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<i32> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"items\": [11, 22, 33, 44]
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            vec![11, 22, 33, 44],
            parsed.items)
    }

    #[test]
    fn when_populated_with_str() {
        const TEXT: &str = "
        {
            \"items\": [
                \"foo\",
                \"bar\",
                \"qux\",
                \"baz\"
            ]
        }";

        let parse_result: serde_json::Result<HasStrVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            vec!["foo".to_string(), "bar".to_string(), "qux".to_string(), "baz".to_string()],
            parsed.items)
    }

    #[test]
    fn when_value_null() {
        const TEXT: &str = "
        {
            \"items\": [
                null
            ]
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);

        let parsed = parse_result.unwrap_err();
        assert!(parsed.is_data());

        let message = format!("{:?}", parsed);
        assert!(message.contains("invalid type: null, expected i32"));
    }

}

#[cfg(test)]
mod test_dz_vec_of_string {
    use serde::Deserialize;
    use super::dz_vec;

    #[derive(Debug, Deserialize)]
    struct HasVec {
        #[serde(default, deserialize_with = "dz_vec")]
        pub items: Vec<String>
    }

    #[test]
    fn when_absent() {
        const TEXT: &str = "{}";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_empty() {
        const TEXT: &str = "
        {
            \"items\": []
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_null() {
        const TEXT: &str = "
        {
            \"items\": null
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parsed.items)
    }

    #[test]
    fn when_populated() {
        const TEXT: &str = "
        {
            \"items\": [
                \"foo\",
                \"bar\",
                \"qux\",
                \"baz\"
            ]
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);
        let parsed = parse_result.unwrap();

        assert_eq!(
            vec!["foo".to_string(), "bar".to_string(), "qux".to_string(), "baz".to_string()],
            parsed.items)
    }

    #[test]
    fn when_value_null() {
        const TEXT: &str = "
        {
            \"items\": [
                null
            ]
        }";

        let parse_result: serde_json::Result<HasVec> = serde_json::from_str(TEXT);

        let parsed = parse_result.unwrap_err();
        assert!(parsed.is_data());

        let message = format!("{:?}", parsed);
        assert!(message.contains("invalid type: null, expected a string"));
    }

}
