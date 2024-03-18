use std::fmt::Display;

use crate::constants::{UTF16BE_BOM, UTF16LE_BOM, UTF16_BUFFER_SIZE, UTF8_BOM};

/// Represents the supported encodings.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Encoding {
    Utf8,
    Utf8Bom,
    Utf16Be,
    Utf16Le,
}

impl From<Encoding> for String {
    fn from(encoding: Encoding) -> Self {
        encoding.to_string()
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Utf8 => write!(f, "UTF-8"),
            Encoding::Utf8Bom => write!(f, "UTF-8-BOM"),
            Encoding::Utf16Be => write!(f, "UTF-16-BE"),
            Encoding::Utf16Le => write!(f, "UTF-16-LE"),
        }
    }
}

/// Encodes a [String] into bytes using [Encoding::Utf8]
pub fn to_utf8_bom(s: &String) -> Vec<u8> {
    [UTF8_BOM, s.as_bytes()].concat()
}

/// Encodes a [String] into bytes using [Encoding::Utf16Be]
pub fn to_utf16_be(s: &str) -> Vec<u8> {
    let mut bytes = UTF16BE_BOM.to_vec();
    let mut buffer = [0u16; UTF16_BUFFER_SIZE];
    for c in s.chars() {
        for u16_unit in c.encode_utf16(&mut buffer) {
            bytes.extend_from_slice(u16_unit.to_be_bytes().as_slice())
        }
    }

    bytes
}

/// Encodes a [String] into bytes using [Encoding::Utf16Le]
pub fn to_utf16_le(s: &str) -> Vec<u8> {
    let mut bytes = UTF16LE_BOM.to_vec();
    let mut buffer = [0u16; UTF16_BUFFER_SIZE];
    for c in s.chars() {
        for u16_unit in c.encode_utf16(&mut buffer) {
            bytes.extend_from_slice(u16_unit.to_le_bytes().as_slice())
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{to_utf16_be, to_utf16_le, to_utf8_bom};

    #[test_case("", b"\xEF\xBB\xBF"; "no chars")] // BOM is always added
    #[test_case("Hello!", b"\xEF\xBB\xBF\x48\x65\x6C\x6C\x6F\x21"; "ascii chars (8-bit chars)")]
    #[test_case("éüñç", b"\xEF\xBB\xBF\xC3\xA9\xC3\xBC\xC3\xB1\xC3\xA7"; "latin-1 chars (16-bit chars)")]
    #[test_case("你好", b"\xEF\xBB\xBF\xE4\xBD\xA0\xE5\xA5\xBD"; "mandarin chars (24-bit chars)")]
    #[test_case("🌍🚀", b"\xEF\xBB\xBF\xF0\x9F\x8C\x8D\xF0\x9F\x9A\x80"; "Supplementary Multilingual Plane chars (32-bit chars)")]
    fn test_to_utf8_bom(input: &str, expected_bytes: &[u8]) {
        let bytes = to_utf8_bom(&input.into());
        assert_eq!(bytes, expected_bytes);
    }

    #[test_case("", b"\xFE\xFF"; "no chars")]
    #[test_case("Hello!", b"\xFE\xFF\x00\x48\x00\x65\x00\x6C\x00\x6C\x00\x6F\x00\x21"; "16-bit chars")]
    #[test_case("🌍🚀", b"\xFE\xFF\xD8\x3C\xDF\x0D\xD8\x3D\xDE\x80"; "32-bit chars with BE BOM")]
    #[test_case("Hello! 😊", b"\xFE\xFF\x00\x48\x00\x65\x00\x6C\x00\x6C\x00\x6F\x00\x21\x00\x20\xD8\x3D\xDE\x0A"; "mixed-length chars with BE BOM")]
    fn test_to_utf16_be(input: &str, expected_bytes: &[u8]) {
        let bytes = to_utf16_be(input);
        assert_eq!(bytes, expected_bytes);
    }

    #[test_case("", b"\xFF\xFE"; "no chars")]
    #[test_case("Hello!", b"\xFF\xFE\x48\x00\x65\x00\x6C\x00\x6C\x00\x6F\x00\x21\x00"; "16-bit chars")]
    #[test_case("🌍🚀", b"\xFF\xFE\x3C\xD8\x0D\xDF\x3D\xD8\x80\xDE"; "32-bit chars with BE BOM")]
    #[test_case("Hello! 😊", b"\xFF\xFE\x48\x00\x65\x00\x6C\x00\x6C\x00\x6F\x00\x21\x00\x20\x00\x3D\xD8\x0A\xDE"; "mixed-length chars with BE BOM")]
    fn test_to_utf16_le(input: &str, expected_bytes: &[u8]) {
        let bytes = to_utf16_le(input);
        assert_eq!(bytes, expected_bytes);
    }
}
