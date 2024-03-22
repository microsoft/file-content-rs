use std::fs;
use std::io::Read;
use std::path::Path;

use crate::constants::{
    BINARY_DETECTION_THRESHOLD, UTF16BE_BOM, UTF16LE_BOM, UTF16_BOM_LENGTH, UTF8_BOM,
    UTF8_BOM_LENGTH, ZERO_BYTE,
};
use crate::encoding::Encoding;
use crate::utf16::{to_u16_be, to_u16_le, UnevenByteSequenceError};
use crate::FileError;

#[derive(Debug, PartialEq)]
pub struct TextData {
    pub data: String,
    pub encoding: Encoding,
}

#[derive(Debug, thiserror::Error)]
pub enum TextDataError {
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    FromUtf16(#[from] std::string::FromUtf16Error),

    #[error(transparent)]
    UnevenByteSequence(#[from] UnevenByteSequenceError),

    #[error("File content is binary")]
    Binary,
}

impl TryFrom<&Path> for TextData {
    type Error = FileError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut file = fs::File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        file.read_to_end(&mut bytes)?;
        Ok(TextData::try_from(bytes.as_slice())?)
    }
}

impl TryFrom<&[u8]> for TextData {
    type Error = TextDataError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.starts_with(UTF8_BOM) {
            Ok(TextData {
                data: String::from_utf8(bytes[UTF8_BOM_LENGTH..].to_vec())?,
                encoding: Encoding::Utf8Bom,
            })
        } else if bytes.starts_with(UTF16BE_BOM) {
            Ok(TextData {
                data: String::from_utf16(&to_u16_be(&bytes[UTF16_BOM_LENGTH..])?)?,
                encoding: Encoding::Utf16Be,
            })
        } else if bytes.starts_with(UTF16LE_BOM) {
            Ok(TextData {
                data: String::from_utf16(&to_u16_le(&bytes[UTF16_BOM_LENGTH..])?)?,
                encoding: Encoding::Utf16Le,
            })
        } else if is_binary(bytes) {
            Err(TextDataError::Binary)
        } else {
            Ok(TextData {
                data: String::from_utf8(bytes.to_vec())?,
                encoding: Encoding::Utf8,
            })
        }
    }
}

/// Returns true if it finds a zero-byte within the first 8 thousand bytes (same as Git)
fn is_binary(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .take(BINARY_DETECTION_THRESHOLD)
        .any(|b| *b == ZERO_BYTE)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{
        encoding::Encoding,
        text_data::{TextData, TextDataError},
    };

    const UTF8BOM_EMPTY_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF8BOM/empty"
    ));
    const UTF8BOM_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF8BOM/ascii"
    ));
    const UTF8BOM_UNICODE_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF8BOM/unicode"
    ));

    const UTF16BE_EMPTY_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16BE/empty"
    ));
    const UTF16BE_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16BE/ascii"
    ));
    const UTF16BE_UNICODE_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16BE/unicode"
    ));

    const UTF16LE_EMPTY_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16LE/empty"
    ));
    const UTF16LE_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16LE/ascii"
    ));
    const UTF16LE_UNICODE_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16LE/unicode"
    ));

    #[test_case(""; "No content")]
    #[test_case("Hello!"; "ASCII chars")]
    #[test_case("Hello! 你好! 🌍"; "Unicode chars")]
    fn from_valid_utf8(input: &str) {
        let subject = TextData::try_from(input.as_bytes()).expect("Should pass");
        let expected = TextData {
            data: input.into(),
            encoding: Encoding::Utf8,
        };

        assert_eq!(subject, expected);
    }

    // Overlong encoding refers to using more bytes than necessary for a character [https://codedocs.org/what-is/utf-8#Overlong_encodings]
    #[test_case(b"\xC1\x80"; "Overlong encoding")]
    #[test_case(b"\x80\xA2"; "Invalid start byte")]
    #[test_case(b"\xE0\xA4"; "Incomplete sequence")]
    #[test_case(b"\xF4\x90\x80\x80"; "Code points above maximum")]
    fn from_invalid_utf8(bytes: &[u8]) {
        let subject = TextData::try_from(bytes);

        assert!(matches!(subject, Err(TextDataError::FromUtf8(_))))
    }

    #[test_case(UTF8BOM_EMPTY_CONTENT, ""; "No content")]
    #[test_case(UTF8BOM_ASCII_CONTENT, "Hello!"; "ASCII chars")]
    #[test_case(UTF8BOM_UNICODE_CONTENT, "Hello! 你好! 🌍"; "Unicode chars")]
    fn from_valid_utf8_with_bom(bytes: &[u8], content: &str) {
        let subject = TextData::try_from(bytes).expect("Should pass");
        let expected = TextData {
            data: content.into(),
            encoding: Encoding::Utf8Bom,
        };

        assert_eq!(subject, expected);
    }

    #[test_case(b"\xEF\xBB\xBF\xC1\x80"; "Overlong encoding")]
    #[test_case(b"\xEF\xBB\xBF\x80\xA2"; "Invalid start byte")]
    #[test_case(b"\xEF\xBB\xBF\xE0\xA4"; "Incomplete sequence")]
    #[test_case(b"\xEF\xBB\xBF\xF4\x90\x80\x80"; "Code points above maximum")]
    fn from_invalid_utf8_with_bom(bytes: &[u8]) {
        let subject = TextData::try_from(bytes);

        assert!(matches!(subject, Err(TextDataError::FromUtf8(_))));
    }

    #[test_case(UTF16BE_EMPTY_CONTENT, ""; "No content")]
    #[test_case(UTF16BE_ASCII_CONTENT, "Hello!"; "ASCII chars")]
    #[test_case(UTF16BE_UNICODE_CONTENT, "Hello! 你好! 🌍"; "Unicode chars")]
    fn from_valid_utf16be(bytes: &[u8], content: &str) {
        let subject = TextData::try_from(bytes).expect("Should pass");
        let expected = TextData {
            data: content.into(),
            encoding: Encoding::Utf16Be,
        };

        assert_eq!(subject, expected);
    }

    #[test_case(b"\xFE\xFF\xD8\xA5"; "Invalid high surrogate")]
    #[test_case(b"\xFE\xFF\xDC\xA5"; "Invalid low surrogate")]
    #[test_case(b"\xFE\xFF\xD8\x3D"; "Incomplete sequence")]
    #[test_case(b"\xFE\xFF\xDB\xFF\xFF\xFF"; "Code points above maximum")]
    fn from_invalid_utf16be(bytes: &[u8]) {
        let subject = TextData::try_from(bytes);

        assert!(matches!(subject, Err(TextDataError::FromUtf16(_))));
    }

    #[test_case(UTF16LE_EMPTY_CONTENT, ""; "No content")]
    #[test_case(UTF16LE_ASCII_CONTENT, "Hello!"; "ASCII chars")]
    #[test_case(UTF16LE_UNICODE_CONTENT, "Hello! 你好! 🌍"; "Unicode chars")]
    fn from_valid_utf16le(bytes: &[u8], content: &str) {
        let subject = TextData::try_from(bytes).expect("Should pass");
        let expected = TextData {
            data: content.into(),
            encoding: Encoding::Utf16Le,
        };

        assert_eq!(subject, expected);
    }

    #[test_case(b"\xFF\xFE\xA5\xD8"; "Invalid high surrogate")]
    #[test_case(b"\xFF\xFE\xA5\xDC"; "Invalid low surrogate")]
    #[test_case(b"\xFF\xFE\x3D\xD8"; "Incomplete sequence")]
    #[test_case(b"\xFF\xFE\xFF\xFF\xFF\xDB"; "Code points above maximum")]
    fn from_invalid_utf16le(bytes: &[u8]) {
        let subject = TextData::try_from(bytes);

        assert!(matches!(subject, Err(TextDataError::FromUtf16(_))));
    }

    #[test_case(b"\0"; "Single zero-byte")]
    #[test_case(b"\x12\x34\0"; "Trailing zero-byte")]
    #[test_case(b"\0\x12\x34"; "Zero-byte at start")]
    fn from_binary(bytes: &[u8]) {
        let subject = TextData::try_from(bytes);

        assert!(matches!(subject, Err(TextDataError::Binary)));
    }
}
