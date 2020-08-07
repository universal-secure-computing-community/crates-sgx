use tokio::stream::{self, Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_test::{assert_pending, assert_ready, task};
use std::string::ToString;
use crates_unittest::test_case;

#[crates_unittest::test]
async fn basic_usage() {
    let one = stream::iter(vec![1, 2, 3]);
    let two = stream::iter(vec![4, 5, 6]);

    let mut stream = one.chain(two);

    assert_eq!(stream.size_hint(), (6, Some(6)));
    assert_eq!(stream.next().await, Some(1));

    assert_eq!(stream.size_hint(), (5, Some(5)));
    assert_eq!(stream.next().await, Some(2));

    assert_eq!(stream.size_hint(), (4, Some(4)));
    assert_eq!(stream.next().await, Some(3));

    assert_eq!(stream.size_hint(), (3, Some(3)));
    assert_eq!(stream.next().await, Some(4));

    assert_eq!(stream.size_hint(), (2, Some(2)));
    assert_eq!(stream.next().await, Some(5));

    assert_eq!(stream.size_hint(), (1, Some(1)));
    assert_eq!(stream.next().await, Some(6));

    assert_eq!(stream.size_hint(), (0, Some(0)));
    assert_eq!(stream.next().await, None);

    assert_eq!(stream.size_hint(), (0, Some(0)));
    assert_eq!(stream.next().await, None);
}

#[crates_unittest::test]
async fn pending_first() {
    let (tx1, rx1) = mpsc::unbounded_channel();
    let (tx2, rx2) = mpsc::unbounded_channel();

    let mut stream = task::spawn(rx1.chain(rx2));
    assert_eq!(stream.size_hint(), (0, None));

    assert_pending!(stream.poll_next());

    tx2.send(2).unwrap();
    assert!(!stream.is_woken());

    assert_pending!(stream.poll_next());

    tx1.send(1).unwrap();
    assert!(stream.is_woken());
    assert_eq!(Some(1), assert_ready!(stream.poll_next()));

    assert_pending!(stream.poll_next());

    drop(tx1);

    assert_eq!(stream.size_hint(), (0, None));

    assert!(stream.is_woken());
    assert_eq!(Some(2), assert_ready!(stream.poll_next()));

    assert_eq!(stream.size_hint(), (0, None));

    drop(tx2);

    assert_eq!(stream.size_hint(), (0, None));
    assert_eq!(None, assert_ready!(stream.poll_next()));
}

#[test_case]
fn size_overflow() {
    struct Monster;

    impl tokio::stream::Stream for Monster {
        type Item = ();
        fn poll_next(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<()>> {
            panic!()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::max_value(), Some(usize::max_value()))
        }
    }

    let m1 = Monster;
    let m2 = Monster;
    let m = m1.chain(m2);
    assert_eq!(m.size_hint(), (usize::max_value(), None));
}
