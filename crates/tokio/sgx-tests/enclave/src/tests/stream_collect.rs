use tokio::stream::{self, StreamExt};
use tokio::sync::mpsc;
use tokio_test::{assert_pending, assert_ready, assert_ready_err, assert_ready_ok, task};

use bytes::{Bytes, BytesMut};
use std::string::ToString;
use crates_unittest::test_case;
use std::prelude::v1::*;

#[allow(clippy::let_unit_value)]
#[crates_unittest::test]
async fn empty_unit() {
    // Drains the stream.
    let mut iter = vec![(), (), ()].into_iter();
    let _: () = stream::iter(&mut iter).collect().await;
    assert!(iter.next().is_none());
}

#[crates_unittest::test]
async fn empty_vec() {
    let coll: Vec<u32> = stream::empty().collect().await;
    assert!(coll.is_empty());
}

#[crates_unittest::test]
async fn empty_box_slice() {
    let coll: Box<[u32]> = stream::empty().collect().await;
    assert!(coll.is_empty());
}

#[crates_unittest::test]
async fn empty_bytes() {
    let coll: Bytes = stream::empty::<&[u8]>().collect().await;
    assert!(coll.is_empty());
}

#[crates_unittest::test]
async fn empty_bytes_mut() {
    let coll: BytesMut = stream::empty::<&[u8]>().collect().await;
    assert!(coll.is_empty());
}

#[crates_unittest::test]
async fn empty_string() {
    let coll: String = stream::empty::<&str>().collect().await;
    assert!(coll.is_empty());
}

#[crates_unittest::test]
async fn empty_result() {
    let coll: Result<Vec<u32>, &str> = stream::empty().collect().await;
    assert_eq!(Ok(vec![]), coll);
}

#[crates_unittest::test]
async fn collect_vec_items() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<Vec<i32>>());

    assert_pending!(fut.poll());

    tx.send(1).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send(2).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    drop(tx);
    assert!(fut.is_woken());
    let coll = assert_ready!(fut.poll());
    assert_eq!(vec![1, 2], coll);
}

#[crates_unittest::test]
async fn collect_string_items() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<String>());

    assert_pending!(fut.poll());

    tx.send("hello ".to_string()).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send("world".to_string()).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    drop(tx);
    assert!(fut.is_woken());
    let coll = assert_ready!(fut.poll());
    assert_eq!("hello world", coll);
}

#[crates_unittest::test]
async fn collect_str_items() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<String>());

    assert_pending!(fut.poll());

    tx.send("hello ").unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send("world").unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    drop(tx);
    assert!(fut.is_woken());
    let coll = assert_ready!(fut.poll());
    assert_eq!("hello world", coll);
}

#[crates_unittest::test]
async fn collect_bytes() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<Bytes>());

    assert_pending!(fut.poll());

    tx.send(&b"hello "[..]).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send(&b"world"[..]).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    drop(tx);
    assert!(fut.is_woken());
    let coll = assert_ready!(fut.poll());
    assert_eq!(&b"hello world"[..], coll);
}

#[crates_unittest::test]
async fn collect_results_ok() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<Result<String, &str>>());

    assert_pending!(fut.poll());

    tx.send(Ok("hello ")).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send(Ok("world")).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    drop(tx);
    assert!(fut.is_woken());
    let coll = assert_ready_ok!(fut.poll());
    assert_eq!("hello world", coll);
}

#[crates_unittest::test]
async fn collect_results_err() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut fut = task::spawn(rx.collect::<Result<String, &str>>());

    assert_pending!(fut.poll());

    tx.send(Ok("hello ")).unwrap();
    assert!(fut.is_woken());
    assert_pending!(fut.poll());

    tx.send(Err("oh no")).unwrap();
    assert!(fut.is_woken());
    let err = assert_ready_err!(fut.poll());
    assert_eq!("oh no", err);
}
