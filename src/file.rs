use std::{
    fmt::Display,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    encoding::{to_utf16_be, to_utf16_le, to_utf8_bom, Encoding},
    text_data::TextData,
};

/// An enum that represents the possible contents of a file
/// 
/// - `Encoded`: The content is a string that can be decoded as one of the
/// supported encodings from [Encoding] (held in a [TextData])
/// - `Binary`: The content is a sequence of bytes that cannot be decoded as a string
#[derive(Debug, PartialEq)]
pub enum FileContent {
    Encoded { content: TextData },
    Binary { content: Vec<u8> },
}

impl FileContent {
    pub fn write<T: Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        match self {
            FileContent::Encoded { content } => match content.encoding {
                Encoding::Utf8 => writer.write_all(content.data.as_bytes()),
                Encoding::Utf8Bom => writer.write_all(&to_utf8_bom(&content.data)),
                Encoding::Utf16Be => writer.write_all(&to_utf16_be(&content.data)),
                Encoding::Utf16Le => writer.write_all(&to_utf16_le(&content.data)),
            },
            FileContent::Binary { content } => writer.write_all(content),
        }
    }
}

/// A file representation that can be used to pair a file path with its content.
/// [File] provides convenience methods for working with files on disk, or in memory.
#[derive(Debug, PartialEq)]
pub struct File {
    pub path: PathBuf,
    pub content: FileContent,
}

/// Represents the possible errors that can occur when working with [File] structs.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TextData(#[from] crate::text_data::TextDataError),
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.content {
            FileContent::Encoded { content } => write!(
                f,
                "File: {}\nEncoding: {}\nContent:\n{}",
                self.path.display(),
                content.encoding,
                content.data
            ),
            FileContent::Binary { content } => write!(
                f,
                "File: {}\nEncoding: Binary\nContent:\n{:?}",
                self.path.display(),
                content
            ),
        }
    }
}

impl File {
    /// Create a [File] with the given path and read it's content from the input [std::io::Read].
    /// The encoding is detected as we read the content, and the appropriate [FileContent] is used.
    pub fn new(path: impl Into<PathBuf>, mut input: impl std::io::Read) -> Result<Self, FileError> {
        let mut bytes: Vec<u8> = vec![];
        input.read_to_end(&mut bytes)?;
        let path = path.into();
        let content = TextData::try_from(bytes.as_slice());
        let content = if let Ok(content) = content {
            FileContent::Encoded { content }
        } else {
            FileContent::Binary { content: bytes }
        };

        Ok(File { path, content })
    }

    pub fn new_from_path(path: impl Into<PathBuf>) -> Result<Self, FileError> {
        let path = path.into();
        let reader = std::fs::File::open(&path)?;
        Self::new(path, reader)
    }

    /// Save the content of a file to disk at it's [PathBuf], using the current encoding for the content.
    pub fn save_to_path(&self) -> Result<(), std::io::Error> {
        let mut writer = fs::File::create(&self.path)?;
        self.content.write(&mut writer)
    }
}

/// Read the content and return as a [String] if it can be decoded as one of the supported encodings from [Encoding].
pub fn read_from_reader(mut input: impl Read) -> Result<String, FileError> {
    let mut bytes = vec![];
    input.read_to_end(&mut bytes)?;
    let text_data = TextData::try_from(bytes.as_slice())?;
    Ok(text_data.data)
}

/// Read the contents of a file from the given path and return as a [String] if it can be decoded as one of the supported encodings from [Encoding].
pub fn read_to_string(path: impl AsRef<Path>) -> Result<String, FileError> {
    Ok(TextData::try_from(path.as_ref())?.data)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::encoding::Encoding;
    use crate::file::File;
    use crate::text_data::TextData;
    use crate::FileContent;

    const UTF8BOM_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF8BOM/ascii"
    ));
    const UTF16BE_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16BE/ascii"
    ));
    const UTF16LE_ASCII_CONTENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/UTF16LE/ascii"
    ));

    #[test_case(b"Hello!", Encoding::Utf8)]
    #[test_case(UTF8BOM_ASCII_CONTENT, Encoding::Utf8Bom)]
    #[test_case(UTF16BE_ASCII_CONTENT, Encoding::Utf16Be)]
    #[test_case(UTF16LE_ASCII_CONTENT, Encoding::Utf16Le)]
    fn load_from_encoded_content(bytes: &[u8], encoding: Encoding) {
        let subject = File::new("foo.txt", bytes).expect("Should pass");
        let expected = File {
            path: "foo.txt".into(),
            content: FileContent::Encoded {
                content: TextData {
                    data: "Hello!".into(),
                    encoding,
                },
            },
        };

        assert_eq!(subject, expected);
    }

    #[test]
    fn load_from_binary() {
        let bytes: &[u8] = &[1, 2, 3, 0, 4, 5];
        let subject = File::new("foo.txt", bytes).expect("Should pass");
        let expected = File {
            path: "foo.txt".into(),
            content: FileContent::Binary {
                content: (*bytes).to_vec(),
            },
        };

        assert_eq!(subject, expected);
    }
}
