#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::io::AsyncReadExt;
use tokio_test::assert_ok;
use std::string::ToString;
use crates_unittest::test_case;
use std::prelude::v1::*;

#[crates_unittest::test]
async fn read_exact() {
    let mut buf = Box::new([0; 8]);
    let mut rd: &[u8] = b"hello world";

    let n = assert_ok!(rd.read_exact(&mut buf[..]).await);
    assert_eq!(n, 8);
    assert_eq!(buf[..], b"hello wo"[..]);
}
