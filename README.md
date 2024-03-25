# `file-content` 

![Crates.io Version](https://img.shields.io/crates/v/file-content?style=for-the-badge&color=blue)

A small library for reading file content/text data from disk, or anywhere else.

## Supported Encodings
* `UTF-8`
* `UTF-8-BOM`
* `UTF-16-BE`
* `UTF-16-LE`
* or raw bytes

## Usage

There are two main structs in this crate.
* `file_content::File`: A wrapper around a `PathBuf` and a `file_content::FileContent`.
  
  Use this struct for easily reading file content from disk that may be in any of the supported encodings.

* `file_content::FileContent`: An enum of the kind of content, either `Encoded` or `Binary`. If `Encoded`, the variant holds the encoding that content had, and a `String` representation of it in memory. If `Binary`, then a `Vec<u8>` of the raw data is held.

Example: `read_file.rs` reads a file from disk and prints the path, content type, and content:
```rust
use anyhow::anyhow;
use file_content::File;

fn main() -> anyhow::Result<()> {
    let file = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Usage: read_file <file>"))?;

    let file = File::new_from_path(file)?;

    println!("{}", file);

    Ok(())
}
```

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft 
trademarks or logos is subject to and must follow 
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.
