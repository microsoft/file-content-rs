pub const UTF16BE_BOM: &[u8; 2] = b"\xFE\xFF";
pub const UTF16LE_BOM: &[u8; 2] = b"\xFF\xFE";
pub const UTF8_BOM: &[u8; 3] = b"\xEF\xBB\xBF";

// Maximum buffer size (in 16-bit units) required for encoding a single UTF-16 character.
pub const UTF16_BUFFER_SIZE: usize = 2;

pub const ZERO_BYTE: u8 = 0x00;
pub const BINARY_DETECTION_THRESHOLD: usize = 8_000;

pub const UTF8_BOM_LENGTH: usize = 3;
pub const UTF16_BOM_LENGTH: usize = 2;
