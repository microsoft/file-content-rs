use std::path::PathBuf;

use anyhow::anyhow;
use file_content::File;

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Usage: read_file <file>"))?;

    let path = PathBuf::from(path);
    let file = File::new_from_path(&path)?;

    println!("{file}");

    let content = file_content::read_to_string_from_path(&path)?;
    println!("{content}");

    Ok(())
}
