use anyhow::anyhow;
use file_content::File;

fn main() -> anyhow::Result<()> {
    let file_path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Usage: read_file <file>"))?;

    let file: File = File::new_from_path(&file_path)?;

    println!("{:?}", file);

    let content_only: String = file_content::read_to_string(&file_path)?;

    println!("{content_only}");

    Ok(())
}