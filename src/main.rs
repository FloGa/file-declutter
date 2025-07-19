use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to declutter
    path: PathBuf,

    /// Number of nested subdirectory levels
    #[arg(long, short, default_value_t = 3)]
    levels: usize,

    /// Remove empty directories after moving files
    #[arg(long, short)]
    remove_empty_directories: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    file_declutter::FileDeclutter::new_from_path(args.path)
        .levels(args.levels)
        .declutter_files(args.remove_empty_directories)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
