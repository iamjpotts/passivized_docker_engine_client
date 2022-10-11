use std::fmt::{Display, Formatter};

use serde::Deserialize;

/// Docker Engine returned an error status code, which the client expects
/// to be accompanied by a JSON response body in this specific layout.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct ErrorResponse {
    pub(crate) message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod test_error_response {
    use super::ErrorResponse;

    #[test]
    fn display() {
        let error = ErrorResponse {
            message: "Boom!".to_string()
        };

        let actual = format!("{}", error);

        assert_eq!("Boom!".to_string(), actual);
    }
}