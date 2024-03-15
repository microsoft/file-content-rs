use std::{fmt::Display, fs, path::PathBuf};

use crate::{
    encoding::{decode_utf16be, decode_utf16le, decode_utf8_with_bom, Encoding},
    text_data::TextData,
};

#[derive(Debug, PartialEq)]
pub enum FileContent {
    Encoded { content: TextData },
    Binary { content: Vec<u8> },
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub path: PathBuf,
    pub content: FileContent,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    /// Instantiates a [File] and loads its content into memory, detecting its encoding in the process.
    pub fn new(path: impl Into<PathBuf>, mut input: impl std::io::Read) -> Result<File, Error> {
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

    /// Save the content of a file to disk, using the appropriate encoding for the content.
    pub fn save(&self) -> Result<(), std::io::Error> {
        match &self.content {
            FileContent::Encoded { content } => fs::write(
                &self.path,
                match content.encoding {
                    Encoding::Utf8 => content.data.as_bytes().to_vec(),
                    Encoding::Utf8WithBom => decode_utf8_with_bom(&content.data),
                    Encoding::Utf16Be => decode_utf16be(&content.data),
                    Encoding::Utf16Le => decode_utf16le(&content.data),
                },
            )?,
            FileContent::Binary { content } => {
                fs::write(&self.path, content)?;
            }
        };

        Ok(())
    }
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
    #[test_case(UTF8BOM_ASCII_CONTENT, Encoding::Utf8WithBom)]
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
