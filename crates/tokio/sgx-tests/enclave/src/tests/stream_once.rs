use tokio::stream::{self, Stream, StreamExt};
use std::string::ToString;
use crates_unittest::test_case;

#[crates_unittest::test]
async fn basic_usage() {
    let mut one = stream::once(1);

    assert_eq!(one.size_hint(), (1, Some(1)));
    assert_eq!(Some(1), one.next().await);

    assert_eq!(one.size_hint(), (0, Some(0)));
    assert_eq!(None, one.next().await);
}
