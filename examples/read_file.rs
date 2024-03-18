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
