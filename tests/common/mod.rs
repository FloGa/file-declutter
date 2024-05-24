use std::path::PathBuf;

use lazy_static::lazy_static;

pub const BIN_NAME: &str = "file-declutter";

lazy_static! {
    pub static ref BIN_PATH: PathBuf = assert_cmd::cargo::cargo_bin(BIN_NAME);
}
