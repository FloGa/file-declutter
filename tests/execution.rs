use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use assert_fs::TempDir;
use assert_fs::prelude::*;

mod common;

struct FixtureOptions<'a> {
    files: Vec<&'a str>,
    levels: usize,
    expected_subdirs: usize,
    remove_empty_directories: bool,
}

fn fixture(opts: FixtureOptions) -> Result<()> {
    let temp_dir = TempDir::new()?;
    let dir_orig = temp_dir.child("orig");
    dir_orig.create_dir_all()?;

    for file in opts.files {
        let child = dir_orig.child(file);
        fs::create_dir_all(child.parent().unwrap())?;
        fs::write(child, file)?;
    }

    let dir_work = temp_dir.child("work");
    dir_work.create_dir_all()?;
    dir_work.copy_from(&dir_orig, &["**"])?;

    let create_command = |levels: usize, remove_empty_directories: bool| -> Command {
        let mut cmd = Command::new(&*common::BIN_PATH);

        if remove_empty_directories {
            cmd.arg("--remove-empty-directories");
        }

        cmd.arg("--levels")
            .arg(levels.to_string())
            .arg(dir_work.path());

        cmd
    };

    let files_before = walkdir::WalkDir::new(&dir_orig)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .flatten()
        .filter(|f| f.file_type().is_file())
        .collect::<Vec<_>>();

    create_command(opts.levels, opts.remove_empty_directories)
        .assert()
        .success();

    assert_eq!(
        fs::read_dir(&dir_work)?.flatten().count(),
        opts.expected_subdirs,
        "Expected number of subdirectories does not match"
    );

    let files_between = walkdir::WalkDir::new(&dir_work)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .flatten()
        .filter(|f| f.file_type().is_file())
        .collect::<Vec<_>>();

    assert_eq!(
        files_before.len(),
        files_between.len(),
        "File lists have different sizes"
    );

    for (file_before, file_between) in files_before.iter().zip(&files_between) {
        assert_eq!(
            file_before.file_name(),
            file_between.file_name(),
            "File names do not match"
        );
        assert_eq!(
            fs::read_to_string(file_before.path())?,
            fs::read_to_string(file_between.path())?,
            "File contents do not match"
        );
    }

    create_command(0, true).assert().success();

    let files_after = walkdir::WalkDir::new(&dir_work)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .flatten()
        .filter(|f| f.file_type().is_file())
        .collect::<Vec<_>>();

    assert_eq!(
        files_before.len(),
        files_after.len(),
        "File lists have different sizes"
    );

    for (file_before, file_after) in files_before.iter().zip(&files_after) {
        assert_eq!(
            file_before.file_name(),
            file_after.file_name(),
            "File names do not match"
        );

        if file_before.file_type().is_file() {
            assert_eq!(
                fs::read_to_string(file_before.path())?,
                fs::read_to_string(file_after.path())?,
                "File contents do not match"
            );
        }
    }

    Ok(())
}

#[test]
fn empty_dir() -> Result<()> {
    fixture(FixtureOptions {
        files: vec![],
        levels: 1,
        expected_subdirs: 0,
        remove_empty_directories: false,
    })?;

    Ok(())
}

#[test]
fn short_file_names() -> Result<()> {
    fixture(FixtureOptions {
        files: vec!["00", "01", "10", "11"],
        levels: 1,
        expected_subdirs: 2,
        remove_empty_directories: true,
    })?;

    Ok(())
}

#[test]
fn short_file_names_from_subfolders() -> Result<()> {
    fixture(FixtureOptions {
        files: vec!["a/00", "b/01", "c/10", "d/11"],
        levels: 1,
        expected_subdirs: 2,
        remove_empty_directories: true,
    })?;

    Ok(())
}

#[test]
fn short_file_names_from_subfolders_noclean() -> Result<()> {
    fixture(FixtureOptions {
        files: vec!["a/00", "b/01", "c/10", "d/11"],
        levels: 1,
        expected_subdirs: 6,
        remove_empty_directories: false,
    })?;

    Ok(())
}
