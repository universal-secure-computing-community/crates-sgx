#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::fs;
use tokio_test::assert_ok;
use std::string::ToString;
use crates_unittest::test_case;
#[crates_unittest::test]
async fn path_read_write() {
    let temp = tempdir();
    let dir = temp.path();

    assert_ok!(fs::write(dir.join("bar"), b"bytes").await);
    let out = assert_ok!(fs::read(dir.join("bar")).await);

    assert_eq!(out, b"bytes");
}

fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}
