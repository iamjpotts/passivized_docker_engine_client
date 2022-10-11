use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind::UnexpectedEof;
use std::io::{Read};
use std::string::FromUtf8Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamLine {
    pub kind: StreamKind,
    pub text: String
}

impl StreamLine {

    /// Read the next line. Returns None when the end of the stream (EOF) is reached.
    ///
    /// # Implementation
    ///
    /// https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerAttach
    ///
    /// header := [8]byte{STREAM_TYPE, 0, 0, 0, SIZE1, SIZE2, SIZE3, SIZE4}
    ///
    /// STREAM_TYPE can be:
    ///
    /// 0: stdin (is written on stdout)
    /// 1: stdout
    /// 2: stderr
    ///
    /// SIZE1, SIZE2, SIZE3, SIZE4 are the four bytes of the uint32 size encoded as big endian.
    ///
    /// Following the header is the payload, which is the specified number of bytes of STREAM_TYPE.
    ///
    /// The simplest way to implement this protocol is the following:
    ///
    /// 1. Read 8 bytes.
    /// 2. Choose stdout or stderr depending on the first byte.
    /// 3. Extract the frame size from the last four bytes.
    /// 4. Read the extracted size and output it on the correct output.
    /// 5. Goto 1.
    pub(crate) fn read<R: Read>(mut reader: R) -> Result<Option<StreamLine>, StreamLineReadError> {
        // Read STREAM_TYPE
        let stream_type = match reader.read_u8() {
            Err(other) => {
                return match other.kind() {
                    UnexpectedEof => Ok(None),
                    _ => Err(other.into())
                }
            }
            Ok(value) => {
                value
            }
        };

        // Assumed to be zero and ignored
        reader.read_u8()?;

        // Assumed to be zero and ignored
        reader.read_u8()?;

        // Assumed to be zero and ignored
        reader.read_u8()?;

        // Read SIZE1 thru SIZE4
        let text_length = reader.read_u32::<BigEndian>()?;

        // Read the text frame
        let mut text_bytes = vec![0u8; text_length as usize];
        reader.read_exact(&mut text_bytes)?;

        let line = StreamLine {
            kind: StreamKind::from(stream_type),
            text: String::from_utf8(text_bytes)?
        };

        Ok(Some(line))
    }

    /// Read every remaining stdin/stdout/stderr line available from the reader.
    pub(crate) fn read_all<R: Read>(mut reader: R) -> Result<Vec<StreamLine>, StreamLineReadError> {
        let mut result: Vec<StreamLine> = Vec::new();

        while let Some(line) = Self::read(&mut reader)? {
            result.push(line);
        }

        Ok(result)
    }
}

impl Display for StreamLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// An error occurred while reading or parsing the stream of lines.
///
/// This is never used to represent EOF.
#[derive(Debug)]
pub enum StreamLineReadError {
    Io(std::io::Error),
    Utf8Conversion(FromUtf8Error)
}

impl StreamLineReadError {
    pub fn error_message(&self) -> String {
        match self {
            Self::Io(e) => format!("Stream read IO error: {:?}", e),
            Self::Utf8Conversion(e) => format!("Stream read UTF-8 conversion error: {:?}", e),
        }
    }
}

impl Display for StreamLineReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message())
    }
}

impl From<std::io::Error> for StreamLineReadError {
    fn from(other: std::io::Error) -> Self {
        StreamLineReadError::Io(other)
    }
}

impl From<FromUtf8Error> for StreamLineReadError {
    fn from(other: FromUtf8Error) -> Self {
        StreamLineReadError::Utf8Conversion(other)
    }
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerAttach
///
/// STREAM_TYPE can be:
//
//     0: stdin (is written on stdout)
//     1: stdout
//     2: stderr
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamKind {
    StdIn,
    StdOut,
    StdErr,

    /// Should never be encountered, but is a catch-all for unexpected or future values.
    Other(u8)
}

impl Display for StreamKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::StdIn => "stdin".to_string(),
            Self::StdOut => "stdout".to_string(),
            Self::StdErr => "stderr".to_string(),
            Self::Other(value) => format!("stream {}", value)
        };

        write!(f, "{}", message)
    }
}

impl From<u8> for StreamKind {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::StdIn,
            1 => Self::StdOut,
            2 => Self::StdErr,
            _ => Self::Other(value)
        }
    }
}


#[cfg(test)]
mod test_stream_kind {

    mod fixtures {
        use std::fs::{File};
        use std::io::Read;
        use std::path::PathBuf;

        pub fn path() -> PathBuf {
            PathBuf::from(file!())
                .parent()
                .unwrap()
                .join("test_fixtures")
        }

        pub fn binary<S: Into<String>>(name: S) -> Vec<u8> {
            let file_name = path().join(name.into());

            let mut result: Vec<u8> = Vec::new();

            let mut f = File::open(file_name).unwrap();
            f.read_to_end(&mut result).unwrap();

            result
        }

    }

    mod parses {
        use std::io::Cursor;
        use crate::model::{StreamKind, StreamLine};

        #[test]
        fn hello_world_line() {
            let content: Vec<u8> = super::fixtures::binary("hello-world.bin");
            let mut cursor = Cursor::new(content);

            let line = StreamLine::read(&mut cursor)
                .unwrap()
                .unwrap();

            assert_eq!(StreamKind::StdOut, line.kind);
            assert_eq!("Hello, world.\n".to_string(), line.text);

            let next_line = StreamLine::read(&mut cursor)
                .unwrap();

            assert!(!next_line.is_some());
        }

        #[test]
        fn hello_world_lines() {
            let content: Vec<u8> = super::fixtures::binary("hello-world.bin");
            let mut cursor = Cursor::new(content);

            let lines = StreamLine::read_all(&mut cursor)
                .unwrap();

            assert_eq!(1, lines.len());

            let line0 = lines.get(0).unwrap();
            assert_eq!(StreamKind::StdOut, line0.kind);
            assert_eq!("Hello, world.\n".to_string(), line0.text);
        }
    }
}
