use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;

mod common;

#[test]
fn empty_dir() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let files_before = walkdir::WalkDir::new(&temp_dir)
        .into_iter()
        .filter_entry(|f| f.file_type().is_file())
        .flatten()
        .collect::<Vec<_>>();

    Command::new(&*common::BIN_PATH)
        .arg("--levels")
        .arg(1.to_string())
        .arg(temp_dir.path())
        .assert()
        .success();

    let files_after = walkdir::WalkDir::new(&temp_dir)
        .into_iter()
        .filter_entry(|f| f.file_type().is_file())
        .flatten()
        .collect::<Vec<_>>();

    assert_eq!(
        files_before.len(),
        files_after.len(),
        "File lists have different sizes"
    );

    for (file_before, file_after) in files_before.into_iter().zip(files_after) {
        assert_eq!(
            file_before.file_name(),
            file_after.file_name(),
            "File names do not match"
        );
        assert_eq!(
            fs::read_to_string(file_before.path())?,
            fs::read_to_string(file_after.path())?,
            "File contents do not match"
        );
    }

    Ok(())
}

#[test]
fn short_file_names() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let files = vec!["00", "01", "10", "11"];
    for file in files {
        fs::write(temp_dir.child(file), file)?;
    }

    let files_before = walkdir::WalkDir::new(&temp_dir)
        .into_iter()
        .filter_entry(|f| f.file_type().is_file())
        .flatten()
        .collect::<Vec<_>>();

    Command::new(&*common::BIN_PATH)
        .arg("--levels")
        .arg(1.to_string())
        .arg(temp_dir.path())
        .assert()
        .success();

    assert_eq!(
        fs::read_dir(&temp_dir)?.flatten().count(),
        2,
        "Too many subdirectories"
    );

    let files_after = walkdir::WalkDir::new(&temp_dir)
        .into_iter()
        .filter_entry(|f| f.file_type().is_file())
        .flatten()
        .collect::<Vec<_>>();

    assert_eq!(
        files_before.len(),
        files_after.len(),
        "File lists have different sizes"
    );

    for (file_before, file_after) in files_before.into_iter().zip(files_after) {
        assert_eq!(
            file_before.file_name(),
            file_after.file_name(),
            "File names do not match"
        );
        assert_eq!(
            fs::read_to_string(file_before.path())?,
            fs::read_to_string(file_after.path())?,
            "File contents do not match"
        );
    }

    Ok(())
}
