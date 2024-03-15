#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Uneven length byte sequence")]
pub struct UnevenByteSequenceError;

/// Converts a vector of big-endian encoded bytes into a vector of corresponding u16 values
/// UnevenByteSequenceError will be returned if the input has an uneven length
pub fn to_u16_be(input: &[u8]) -> Result<Vec<u16>, UnevenByteSequenceError> {
    if input.len() % 2 != 0 {
        Err(UnevenByteSequenceError)
    } else {
        Ok(input
            .chunks(2)
            .map(|chunk| {
                let mut buf = [0; 2];
                buf.copy_from_slice(chunk);
                u16::from_be_bytes(buf)
            })
            .collect())
    }
}

/// Converts a vector of little-endian encoded bytes into a vector of corresponding u16 values
/// UnevenByteSequenceError will be returned if the input has an uneven length
pub fn to_u16_le(input: &[u8]) -> Result<Vec<u16>, UnevenByteSequenceError> {
    if input.len() % 2 != 0 {
        Err(UnevenByteSequenceError)
    } else {
        Ok(input
            .chunks(2)
            .map(|chunk| {
                let mut buf = [0; 2];
                buf.copy_from_slice(chunk);
                u16::from_le_bytes(buf)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::utf16::{to_u16_be, to_u16_le, UnevenByteSequenceError};

    #[test_case(b"", &[])]
    #[test_case(b"\x12\x34", &[0x1234])]
    #[test_case(b"\x12\x34\x56\x78", &[0x1234, 0x5678])]
    fn valid_be(bytes: &[u8], expected: &[u16]) {
        let subject = to_u16_be(bytes).expect("Should pass");
        assert_eq!(subject, expected);
    }

    #[test_case(b"", &[])]
    #[test_case(b"\x12\x34", &[0x3412])]
    #[test_case(b"\x12\x34\x56\x78", &[0x3412, 0x7856])]
    fn valid_le(bytes: &[u8], expected: &[u16]) {
        let subject = to_u16_le(bytes).expect("Should pass");
        assert_eq!(subject, expected);
    }

    // The only case that will throw an error while converting to
    // LE or BE is an input of uneven length
    #[test]
    fn invalid_be() {
        let bytes = b"\x12\x34\x56";
        let subject = to_u16_be(bytes);
        assert_eq!(subject, Err(UnevenByteSequenceError));
    }

    #[test]
    fn invalid_le() {
        let bytes = b"\x12\x34\x56";
        let subject = to_u16_le(bytes);
        assert_eq!(subject, Err(UnevenByteSequenceError));
    }
}
