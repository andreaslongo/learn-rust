use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    println!(
        "Lines containing '{}' from '{}':",
        args.pattern,
        args.path.display()
    );

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
    grrs::find_matches(&content, &args.pattern, &mut writer);

    Ok(())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
