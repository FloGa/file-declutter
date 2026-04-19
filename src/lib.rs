//! # File Declutter
//!
//! [![badge github]][url github]
//! [![badge crates.io]][url crates.io]
//! [![badge docs.rs]][url docs.rs]
//! [![badge license]][url license]
//!
//! [badge github]: https://img.shields.io/badge/github-FloGa%2Ffile--declutter-green
//! [badge crates.io]: https://img.shields.io/crates/v/file-declutter
//! [badge docs.rs]: https://img.shields.io/docsrs/file-declutter
//! [badge license]: https://img.shields.io/crates/l/file-declutter
//!
//! [url github]: https://github.com/FloGa/file-declutter
//! [url crates.io]: https://crates.io/crates/file-declutter
//! [url docs.rs]: https://docs.rs/file-declutter
//! [url license]: https://github.com/FloGa/file-declutter/blob/develop/LICENSE
//!
//! > Reorganizes files into nested folders based on their filenames.
//!
//! *File Declutter* is a little command line tool that helps you bring order to
//! your large directories. It can "declutter" a flat list of files by
//! redistributing them into nested subdirectories based on their filenames. It's
//! particularly useful for organizing large numbers of files in a single
//! directory (for example images, documents, etc.) into a more manageable
//! structure.
//!
//! This crate is split into an [Application](#application) part and a
//! [Library](#library) part.
//!
//! ## Motivation
//!
//! The need for this little tool derived from a situation where I was confronted
//! with a flat directory of 500k files and more. Well, to be frank, it was one of
//! my other creations, namely [*DedupeFS*][dedupefs github], which presents files
//! as 1MB chunks, named after their checksums. By using this over my media hard
//! drive, I ended up with so many files in one directory that my file manager
//! just refused to work with it.
//!
//! Since the file names are SHA-1 hashes, they consist of a somewhat evenly
//! distributed sequence of the hexadecimal numbers 0 to f. So, by putting all
//! files that start with 0 into a separate folder, and all that start with 1 into
//! a different folder, and so on, I can already split the number of files per
//! subdirectory by 16. If I repeat this step in each subdirectory, I can split
//! them again by 16.
//!
//! This is one possible scenario where *File Declutter* really comes in handy.
//!
//! [dedupefs github]: https://github.com/FloGa/dedupefs
//!
//! ## Application
//!
//! ### Installation
//!
//! This tool can be installed easily through Cargo via `crates.io`:
//!
//! ```shell
//! cargo install --locked file-declutter
//! ```
//!
//! Please note that the `--locked` flag is necessary here to have the exact same
//! dependencies as when the application was tagged and tested. Without it, you
//! might get more up-to-date versions of dependencies, but you have the risk of
//! undefined and unexpected behavior if the dependencies changed some
//! functionalities. The application might even fail to build if the public API of
//! a dependency changed too much.
//!
//! Alternatively, pre-built binaries can be downloaded from the [GitHub
//! releases][gh-releases] page.
//!
//! [gh-releases]: https://github.com/FloGa/file-declutter/releases
//!
//! ### Usage
//!
//! ```text
//! Usage: file-declutter [OPTIONS] <PATH>
//!
//! Arguments:
//!   <PATH>  Directory to declutter
//!
//! Options:
//!   -l, --levels <LEVELS>           Number of nested subdirectory levels [default: 3]
//!   -r, --remove-empty-directories  Remove empty directories after moving files
//!   -h, --help                      Print help
//!   -V, --version                   Print version
//! ```
//!
//! To declutter a directory into three levels, you would go with:
//!
//! ```shell
//! file-declutter --levels 3 path/to/large-directory
//! ```
//!
//! To "restore" the directory, or rather, flatten a list of files in
//! subdirectories, you can use:
//!
//! ```shell
//! file-declutter --levels 0 --remove-empty-directories path/to/decluttered-directory
//! ```
//!
//! **Warning:** Please note that flattening a directory tree in this way will
//! result in **data loss** when there are files with the same names! So if you
//! have two files `dir/a/123.txt` and `dir/b/123.txt` and you run the above
//! command over `dir`, you will end up with `dir/123.txt`, where the last file
//! move has overwritten the previous ones. The file listing could be in arbitrary
//! order, so you cannot even tell beforehand, which file will "win".
//!
//! ## Library
//!
//! ### Installation
//!
//! To add the `file-clutter` library to your project, you can use:
//!
//! ```shell
//! cargo add file-declutter
//! ```
//!
//! ### Usage
//!
//! The following is a short summary of how this library is intended to be used.
//! The actual functionality lies in a custom Iterator object
//! `FileDeclutterIterator`, but it is not intended to be instantiated directly.
//! Instead, the wrapper `FileDeclutter` should be used.
//!
//! Once the Iterator is created, you can either iterate over its items to receive
//! a list of `(source, target)` tuples, with `source` being the original filename
//! and `target` the "decluttered" one. You can use this to do your own logic over
//! them. Or you can use the `declutter_files` method to do the actual moving of
//! the files.
//!
//! #### Create Iterator from Path
//!
//! This method can be used if you have an actual directory that you want to
//! completely declutter recursively.
//!
//! ```rust no_run
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let files_decluttered = file_declutter::FileDeclutter::new_from_path("/tmp/path")
//!         .levels(1)
//!         .collect::<Vec<_>>();
//!
//!     // If the specified directory contains the files 13.txt and 23.txt, the following tuples
//!     // will be produced:
//!     let files_expected = vec![
//!         (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
//!         (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
//!     ];
//!
//!     assert_eq!(files_expected, files_decluttered);
//! }
//! ```
//!
//! #### Create Iterator from Existing Iterator
//!
//! This method can be used if you have a specific list of files you want to
//! process.
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let files = vec!["13.txt", "23.txt"];
//!     let files_decluttered = file_declutter::FileDeclutter::new_from_iter(files.into_iter())
//!         .levels(1)
//!         .collect::<Vec<_>>();
//!
//!     let files_expected = vec![
//!         (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
//!         (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
//!     ];
//!
//!     assert_eq!(files_expected, files_decluttered);
//! }
//! ```
//!
//! #### Oneshot
//!
//! This method can be used if you have a single file which you want to have a
//! decluttered name for.
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let file = "123456.txt";
//!     let file_decluttered = file_declutter::FileDeclutter::oneshot(file, 3);
//!
//!     let file_expected = PathBuf::from("1/2/3/123456.txt");
//!
//!     assert_eq!(file_expected, file_decluttered);
//! }
//! ```

use std::path::PathBuf;

/// An iterator that transforms a list of file paths into (source, target) pairs, where the target
/// path is a decluttered version based on the filename's characters.
pub struct FileDeclutterIterator<I> {
    inner: I,
    base: PathBuf,
    levels: usize,
}

impl<I> FileDeclutterIterator<I>
where
    I: Iterator<Item = PathBuf>,
{
    /// Sets the base directory into which files will be moved.
    pub fn base<P: Into<PathBuf>>(mut self, base: P) -> Self {
        self.base = base.into();
        self
    }

    /// Sets the number of directory levels to create based on the filename.
    ///
    /// For example, with `levels = 2` and a file named `abcdef.txt`, the target path would include
    /// two subdirectories: `a/b/abcdef.txt`.
    pub fn levels(mut self, levels: usize) -> Self {
        self.levels = levels;
        self
    }

    /// Moves all files to their decluttered target paths.
    ///
    /// If `remove_empty_directories` is `true`, the function will attempt to remove any now-empty
    /// directories after the move operation.
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation or file renaming fails.
    pub fn declutter_files(self, remove_empty_directories: bool) -> anyhow::Result<()> {
        let base = self.base.clone();

        for (source, target) in self {
            std::fs::create_dir_all(&target.parent().unwrap())?;
            std::fs::rename(source, target)?;
        }

        if remove_empty_directories {
            for dir in walkdir::WalkDir::new(base)
                .min_depth(1)
                .contents_first(true)
                .into_iter()
                .filter_entry(|f| f.file_type().is_dir())
                .flatten()
            {
                let dir = dir.into_path();

                if dir.read_dir()?.count() == 0 {
                    // Ignore result, it doesn't matter if deletion fails.
                    let _ = std::fs::remove_dir(dir);
                }
            }
        }

        Ok(())
    }
}

impl<I> Iterator for FileDeclutterIterator<I>
where
    I: Iterator<Item = PathBuf>,
{
    type Item = (PathBuf, PathBuf);

    /// Returns the next `(source, target)` file path pair.
    ///
    /// The target path is derived from the file name by taking the first `levels` characters and
    /// using them as nested directories.
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(move |entry| {
            let sub_dirs = entry.file_name().unwrap().to_string_lossy();
            let sub_dirs = sub_dirs.chars().take(self.levels).map(String::from);

            let mut target_path = self.base.clone();
            for sub_dir in sub_dirs {
                target_path.push(sub_dir);
            }
            target_path.push(entry.file_name().unwrap());

            (entry, target_path)
        })
    }
}

/// Entry point for creating decluttering iterators or computing decluttered paths.
pub struct FileDeclutter;

impl FileDeclutter {
    /// Creates a `FileDeclutterIterator` from an arbitrary iterator over file paths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::PathBuf;
    /// let files = vec!["13.txt", "23.txt"];
    /// let files_decluttered = file_declutter::FileDeclutter::new_from_iter(files.into_iter())
    ///     .levels(1)
    ///     .collect::<Vec<_>>();
    ///
    /// let files_expected = vec![
    ///     (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
    ///     (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
    /// ];
    ///
    /// assert_eq!(files_expected, files_decluttered);
    /// ```
    pub fn new_from_iter(
        iter: impl Iterator<Item = impl Into<PathBuf>>,
    ) -> FileDeclutterIterator<impl Iterator<Item = PathBuf>> {
        FileDeclutterIterator {
            inner: iter.map(Into::into),
            base: Default::default(),
            levels: Default::default(),
        }
    }

    /// Creates a `FileDeclutterIterator` by recursively collecting all files under a given
    /// directory and setting this directory as the base.
    ///
    /// # Examples
    ///
    /// ```rust no_run
    /// # use std::path::PathBuf;
    /// let files_decluttered = file_declutter::FileDeclutter::new_from_path("/tmp/path")
    ///     .levels(1)
    ///     .collect::<Vec<_>>();
    ///
    /// // If the specified directory contains the files 13.txt and 23.txt, the following tuples
    /// // will be produced:
    /// let files_expected = vec![
    ///     (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
    ///     (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
    /// ];
    ///
    /// assert_eq!(files_expected, files_decluttered);
    /// ```
    pub fn new_from_path(
        base: impl Into<PathBuf>,
    ) -> FileDeclutterIterator<impl Iterator<Item = PathBuf>> {
        let base = base.into();

        let iter = walkdir::WalkDir::new(&base)
            .min_depth(1)
            .into_iter()
            .flatten()
            .filter(|f| f.file_type().is_file())
            .map(|entry| entry.into_path());

        FileDeclutter::new_from_iter(iter).base(base)
    }

    /// Computes the decluttered path of a single file without moving it.
    ///
    /// # Arguments
    ///
    /// - `file`: Path to the input file.
    /// - `levels`: Number of subdirectory levels to use.
    ///
    /// # Returns
    ///
    /// A `PathBuf` representing the target decluttered location.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::path::PathBuf;
    /// let file = "123456.txt";
    /// let file_decluttered = file_declutter::FileDeclutter::oneshot(file, 3);
    ///
    /// let file_expected = PathBuf::from("1/2/3/123456.txt");
    ///
    /// assert_eq!(file_expected, file_decluttered);
    /// ```
    pub fn oneshot(file: impl Into<PathBuf>, levels: usize) -> PathBuf {
        let iter = std::iter::once(file.into());
        FileDeclutter::new_from_iter(iter)
            .levels(levels)
            .next()
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
    use rand::RngExt;

    use super::*;

    #[test]
    fn decluttered_from_path_file_names_same() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        let mut rng = rand::rng();
        for _ in 0..100 {
            let mut file_name = rng
                .random_range(1_000_000_000u64..10_000_000_000u64)
                .to_string();

            if rng.random_bool(0.25) {
                file_name = format!("subdir/{file_name}");
            }

            let child = temp_dir.child(file_name);
            child.touch()?;
        }

        for (source, target) in FileDeclutter::new_from_path(temp_dir.to_path_buf()).levels(1) {
            assert_ne!(source.parent(), target.parent());
            assert_eq!(source.file_name(), target.file_name());
        }

        Ok(())
    }

    #[test]
    fn decluttered_from_iter_file_names_same() -> anyhow::Result<()> {
        let mut rng = rand::rng();
        let files = (0..100).map(move |_| {
            let mut file_name = rng
                .random_range(1_000_000_000u64..10_000_000_000u64)
                .to_string();

            if rng.random_bool(0.25) {
                file_name = format!("subdir/{file_name}");
            }

            file_name
        });

        for (source, target) in FileDeclutter::new_from_iter(files).levels(1) {
            assert_ne!(source.parent(), target.parent());
            assert_eq!(source.file_name(), target.file_name());
        }

        Ok(())
    }

    #[test]
    fn oneshot() -> anyhow::Result<()> {
        let source = PathBuf::from("123456");
        let target_expected = PathBuf::from("1/2/3/123456");

        let target_actual = FileDeclutter::oneshot(&source, 3);

        assert_eq!(target_actual, target_expected);

        Ok(())
    }
}
