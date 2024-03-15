#[cfg(test)]
mod file_io_tests {
    use file_content::{Encoding, File, FileContent, TextData};
    use std::fs;
    use test_case::test_case;

    const FILE_CONTENT: &str = "Hello! 你好! 🌍";
    const ENCODED_FILES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data");

    #[test_case("UTF8/unicode", Encoding::Utf8; "UTF-8")]
    #[test_case("UTF8BOM/unicode", Encoding::Utf8WithBom; "UTF-8WithBom")]
    #[test_case("UTF16BE/unicode", Encoding::Utf16Be; "UTF-16BE")]
    #[test_case("UTF16LE/unicode", Encoding::Utf16Le; "UTF-16LE")]
    fn save_encoded_content(path: &str, encoding: Encoding) -> anyhow::Result<()> {
        let path = format!("{ENCODED_FILES_ROOT}/{path}");

        let expected_bytes = fs::read(&path)?;

        let file_content = File {
            path: path.clone().into(),
            content: FileContent::Encoded {
                content: TextData {
                    data: FILE_CONTENT.into(),
                    encoding,
                },
            },
        };

        file_content.save()?;

        let bytes_after_saving = fs::read(&path)?;

        assert_eq!(bytes_after_saving, expected_bytes);

        Ok(())
    }

    #[test]
    fn save_binary_content() -> anyhow::Result<()> {
        let bytes: &[u8] = &[1, 2, 3, 0, 4, 5];
        let path = format!("{ENCODED_FILES_ROOT}/Binary/binary");

        let file_content = File {
            path: path.clone().into(),
            content: FileContent::Binary {
                content: bytes.to_vec(),
            },
        };

        file_content.save()?;

        let bytes_after_saving = fs::read(&path)?;

        assert_eq!(bytes, bytes_after_saving);

        Ok(())
    }
}
