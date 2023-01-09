

/// Convenience wrapper over base64 crate's breaking changes
pub(crate) fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;

    STANDARD.encode(input)
}

/// Some functions convert/extract input parameters that represent failure information.
/// That information is converted to an error result type.
///
/// However, the conversion itself can fail, also producing an error, of the same type.
///
/// Accept a result where the success (Ok) and failure (Err) cases are the same type,
/// and unwrap the error value unwrapped out of the Result type.
pub(crate) fn converge<A>(result: Result<A, A>) -> A {
    match result {
        Err(a) => a,
        Ok(a) => a
    }
}

#[cfg(test)]
mod test_base64_encode {
    use super::base64_encode;

    //noinspection SpellCheckingInspection
    #[test]
    fn encodes() {
        let input: [u8; 4] = [2, 3, 4, 5];

        let expected = "AgMEBQ==";
        assert_eq!(expected, base64_encode(input));
    }
}

#[cfg(test)]
mod test_converge {
    use super::converge;

    #[test]
    fn failure() {
        let result: Result<String, String> = Err("boom".into());

        assert_eq!("boom", converge(result));
    }

    #[test]
    fn success() {
        let result: Result<i32, i32> = Ok(123);

        assert_eq!(123, converge(result));
    }

}