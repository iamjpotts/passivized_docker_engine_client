use std::fmt::{Display, Formatter};
use crate::errors::{DecCreateError, DecUseError};

/// Aggregator for converging creation and usage errors into a single,
/// convenient error type.
///
/// Defines automatic Into conversions for each of its inner failure cases.
///
/// Use of this type is not required, but may be convenient for call sites
/// that both create a DockerEngineClient and use it.
#[derive(Debug, thiserror::Error)]
pub enum DecError {
    DuringCreation(DecCreateError),
    DuringUse(DecUseError)
}

impl From<DecCreateError> for DecError {
    fn from(other: DecCreateError) -> Self {
        DecError::DuringCreation(other)
    }
}

impl From<DecUseError> for DecError {
    fn from(other: DecUseError) -> Self {
        DecError::DuringUse(other)
    }
}

impl Display for DecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecError::DuringCreation(inner) =>
                write!(f, "{}", inner),
            DecError::DuringUse(inner) =>
                write!(f, "{}", inner),
        }
    }
}

#[cfg(test)]
mod test_dec_error_display {
    use crate::errors::{DecCreateError, DecError, DecUseError};

    #[test]
    fn during_creation() {
        let creation_error = DecCreateError::UnsupportedUrlScheme;
        let expected = format!("{}", creation_error);

        let error = DecError::from(creation_error);

        assert_eq!(expected, format!("{}", error))
    }

    #[test]
    fn during_use() {
        let use_error = DecUseError::NotFound { message: "where is it".into() };
        let expected = format!("{}", use_error);

        let error = DecError::from(use_error);

        assert_eq!(expected, format!("{}", error))
    }
}