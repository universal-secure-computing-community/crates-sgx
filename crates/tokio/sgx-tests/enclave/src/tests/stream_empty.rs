use tokio::stream::{self, Stream, StreamExt};
use std::string::ToString;
use crates_unittest::test_case;
#[crates_unittest::test]
async fn basic_usage() {
    let mut stream = stream::empty::<i32>();

    for _ in 0..2 {
        assert_eq!(stream.size_hint(), (0, Some(0)));
        assert_eq!(None, stream.next().await);
    }
}
