#![allow(dead_code)]  // Each test suite compiles separately; not all test suites use all test utilities.

use std::fmt::Display;
use std::path::PathBuf;

pub mod certs;
pub mod content_type;
pub mod images;

pub fn label_key() -> String {
    PathBuf::from(file!())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn label_value() -> String {
    "true".to_string()
}

pub fn random_name<S: Into<String> + Display>(prefix: S) -> String {
    let suffix: u64 = rand::random();
    format!("{}_{}", prefix, suffix)
}
