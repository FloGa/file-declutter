# File Declutter

[![badge github]][url github]
[![badge crates.io]][url crates.io]
[![badge docs.rs]][url docs.rs]
[![badge license]][url license]

[badge github]: https://img.shields.io/badge/github-FloGa%2Ffile--declutter-green
[badge crates.io]: https://img.shields.io/crates/v/file-declutter
[badge docs.rs]: https://img.shields.io/docsrs/file-declutter
[badge license]: https://img.shields.io/crates/l/file-declutter

[url github]: https://github.com/FloGa/file-declutter
[url crates.io]: https://crates.io/crates/file-declutter
[url docs.rs]: https://docs.rs/file-declutter
[url license]: https://github.com/FloGa/file-declutter/blob/develop/LICENSE

> Reorganizes files into nested folders based on their filenames.

*File Declutter* is a little command line tool that helps you bring order to
your large directories. It can "declutter" a flat list of files by
redistributing them into nested subdirectories based on their filenames. It's
particularly useful for organizing large numbers of files in a single
directory (for example images, documents, etc.) into a more manageable
structure.

This crate is split into an [Application](#application) part and a
[Library](#library) part.

## Motivation

The need for this little tool derived from a situation where I was confronted
with a flat directory of 500k files and more. Well, to be frank, it was one of
my other creations, namely [*DedupeFS*][dedupefs github], which presents files
as 1MB chunks, named after their checksums. By using this over my media hard
drive, I ended up with so many files in one directory that my file manager
just refused to work with it.

Since the file names are SHA-1 hashes, they consist of a somewhat evenly
distributed sequence of the hexadecimal numbers 0 to f. So, by putting all
files that start with 0 into a separate folder, and all that start with 1 into
a different folder, and so on, I can already split the number of files per
subdirectory by 16. If I repeat this step in each subdirectory, I can split
them again by 16.

This is one possible scenario where *File Declutter* really comes in handy.

[dedupefs github]: https://github.com/FloGa/dedupefs

## Application

### Installation

This tool can be installed easily through Cargo via `crates.io`:

```shell
cargo install --locked file-declutter
```

Please note that the `--locked` flag is necessary here to have the exact same
dependencies as when the application was tagged and tested. Without it, you
might get more up-to-date versions of dependencies, but you have the risk of
undefined and unexpected behavior if the dependencies changed some
functionalities. The application might even fail to build if the public API of
a dependency changed too much.

Alternatively, pre-built binaries can be downloaded from the [GitHub
releases][gh-releases] page.

[gh-releases]: https://github.com/FloGa/file-declutter/releases

### Usage

<!--% !cargo --quiet run -- --help | tail -n+3 %-->

```text
Usage: file-declutter [OPTIONS] <PATH>

Arguments:
  <PATH>  Directory to declutter

Options:
  -l, --levels <LEVELS>           Number of nested subdirectory levels [default: 3]
  -r, --remove-empty-directories  Remove empty directories after moving files
  -h, --help                      Print help
  -V, --version                   Print version
```

To declutter a directory into three levels, you would go with:

```shell
file-declutter --levels 3 path/to/large-directory
```

To "restore" the directory, or rather, flatten a list of files in
subdirectories, you can use:

```shell
file-declutter --levels 0 --remove-empty-directories path/to/decluttered-directory
```

**Warning:** Please note that flattening a directory tree in this way will
result in **data loss** when there are files with the same names! So if you
have two files `dir/a/123.txt` and `dir/b/123.txt` and you run the above
command over `dir`, you will end up with `dir/123.txt`, where the last file
move has overwritten the previous ones. The file listing could be in arbitrary
order, so you cannot even tell beforehand, which file will "win".

## Library

### Installation

To add the `file-clutter` library to your project, you can use:

```shell
cargo add file-declutter
```

### Usage

The following is a short summary of how this library is intended to be used.
The actual functionality lies in a custom Iterator object
`FileDeclutterIterator`, but it is not intended to be instantiated directly.
Instead, the wrapper `FileDeclutter` should be used.

Once the Iterator is created, you can either iterate over its items to receive
a list of `(source, target)` tuples, with `source` being the original filename
and `target` the "decluttered" one. You can use this to do your own logic over
them. Or you can use the `declutter_files` method to do the actual moving of
the files.

#### Create Iterator from Path

This method can be used if you have an actual directory that you want to
completely declutter recursively.

```rust no_run
use std::path::PathBuf;

fn main() {
    let files_decluttered = file_declutter::FileDeclutter::new_from_path("/tmp/path")
        .levels(1)
        .collect::<Vec<_>>();

    // If the specified directory contains the files 13.txt and 23.txt, the following tuples
    // will be produced:
    let files_expected = vec![
        (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
        (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
    ];

    assert_eq!(files_expected, files_decluttered);
}
```

#### Create Iterator from Existing Iterator

This method can be used if you have a specific list of files you want to
process.

```rust
use std::path::PathBuf;

fn main() {
    let files = vec!["13.txt", "23.txt"];
    let files_decluttered = file_declutter::FileDeclutter::new_from_iter(files.into_iter())
        .levels(1)
        .collect::<Vec<_>>();

    let files_expected = vec![
        (PathBuf::from("13.txt"), PathBuf::from("1/13.txt")),
        (PathBuf::from("23.txt"), PathBuf::from("2/23.txt")),
    ];

    assert_eq!(files_expected, files_decluttered);
}
```

#### Oneshot

This method can be used if you have a single file which you want to have a
decluttered name for.

```rust
use std::path::PathBuf;

fn main() {
    let file = "123456.txt";
    let file_decluttered = file_declutter::FileDeclutter::oneshot(file, 3);

    let file_expected = PathBuf::from("1/2/3/123456.txt");

    assert_eq!(file_expected, file_decluttered);
}
```
