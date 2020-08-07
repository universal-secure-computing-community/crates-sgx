use std::string::ToString;
use crates_unittest::test_case;
#[cfg(feature = "alloc")]
#[test_case]
fn is_terminated() {
    use futures::future;
    use futures::stream::{FusedStream, FuturesUnordered, StreamExt};
    use futures::task::Poll;
    use futures_test::task::noop_context;

    let mut cx = noop_context();
    let mut tasks = FuturesUnordered::new();

    assert_eq!(tasks.is_terminated(), false);
    assert_eq!(tasks.poll_next_unpin(&mut cx), Poll::Ready(None));
    assert_eq!(tasks.is_terminated(), true);

    // Test that the sentinel value doesn't leak
    assert_eq!(tasks.is_empty(), true);
    assert_eq!(tasks.len(), 0);
    assert_eq!(tasks.iter_mut().len(), 0);

    tasks.push(future::ready(1));

    assert_eq!(tasks.is_empty(), false);
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks.iter_mut().len(), 1);

    assert_eq!(tasks.is_terminated(), false);
    assert_eq!(tasks.poll_next_unpin(&mut cx), Poll::Ready(Some(1)));
    assert_eq!(tasks.is_terminated(), false);
    assert_eq!(tasks.poll_next_unpin(&mut cx), Poll::Ready(None));
    assert_eq!(tasks.is_terminated(), true);
}

#[cfg(all(feature = "alloc", feature = "executor"))]
#[test_case]
fn works_1() {
    use futures::channel::oneshot;
    use futures::executor::block_on_stream;
    use futures::stream::FuturesUnordered;

    let (a_tx, a_rx) = oneshot::channel::<i32>();
    let (b_tx, b_rx) = oneshot::channel::<i32>();
    let (c_tx, c_rx) = oneshot::channel::<i32>();

    let mut iter = block_on_stream(
        vec![a_rx, b_rx, c_rx]
            .into_iter()
            .collect::<FuturesUnordered<_>>(),
    );

    b_tx.send(99).unwrap();
    assert_eq!(Some(Ok(99)), iter.next());

    a_tx.send(33).unwrap();
    c_tx.send(33).unwrap();
    assert_eq!(Some(Ok(33)), iter.next());
    assert_eq!(Some(Ok(33)), iter.next());
    assert_eq!(None, iter.next());
}

#[cfg(feature = "alloc")]
#[test_case]
fn works_2() {
    use futures::channel::oneshot;
    use futures::future::{join, FutureExt};
    use futures::stream::{FuturesUnordered, StreamExt};
    use futures::task::Poll;
    use futures_test::task::noop_context;

    let (a_tx, a_rx) = oneshot::channel::<i32>();
    let (b_tx, b_rx) = oneshot::channel::<i32>();
    let (c_tx, c_rx) = oneshot::channel::<i32>();

    let mut stream = vec![
        a_rx.boxed(),
        join(b_rx, c_rx).map(|(a, b)| Ok(a? + b?)).boxed(),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    a_tx.send(9).unwrap();
    b_tx.send(10).unwrap();

    let mut cx = noop_context();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(9))));
    c_tx.send(20).unwrap();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(30))));
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(None));
}

#[cfg(feature = "executor")]
#[test_case]
fn from_iterator() {
    use futures::executor::block_on;
    use futures::future;
    use futures::stream::{FuturesUnordered, StreamExt};
    use std::prelude::v1::*;
    
    let stream = vec![
        future::ready::<i32>(1),
        future::ready::<i32>(2),
        future::ready::<i32>(3),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();
    assert_eq!(stream.len(), 3);
    assert_eq!(block_on(stream.collect::<Vec<_>>()), vec![1, 2, 3]);
}

#[cfg(feature = "alloc")]
#[test_case]
fn finished_future() {
    use std::marker::Unpin;
    use futures::channel::oneshot;
    use futures::future::{self, Future, FutureExt};
    use futures::stream::{FuturesUnordered, StreamExt};
    use futures_test::task::noop_context;
    use std::prelude::v1::*;
    let (_a_tx, a_rx) = oneshot::channel::<i32>();
    let (b_tx, b_rx) = oneshot::channel::<i32>();
    let (c_tx, c_rx) = oneshot::channel::<i32>();

    let mut stream = vec![
        Box::new(a_rx) as Box<dyn Future<Output = Result<_, _>> + Unpin>,
        Box::new(future::select(b_rx, c_rx).map(|e| e.factor_first().0)) as _,
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    let cx = &mut noop_context();
    for _ in 0..10 {
        assert!(stream.poll_next_unpin(cx).is_pending());
    }

    b_tx.send(12).unwrap();
    c_tx.send(3).unwrap();
    assert!(stream.poll_next_unpin(cx).is_ready());
    assert!(stream.poll_next_unpin(cx).is_pending());
    assert!(stream.poll_next_unpin(cx).is_pending());
}

#[cfg(all(feature = "alloc", feature = "executor"))]
#[test_case]
fn iter_mut_cancel() {
    use futures::channel::oneshot;
    use futures::executor::block_on_stream;
    use futures::stream::FuturesUnordered;
    use std::prelude::v1::*;
    let (a_tx, a_rx) = oneshot::channel::<i32>();
    let (b_tx, b_rx) = oneshot::channel::<i32>();
    let (c_tx, c_rx) = oneshot::channel::<i32>();

    let mut stream = vec![a_rx, b_rx, c_rx]
        .into_iter()
        .collect::<FuturesUnordered<_>>();

    for rx in stream.iter_mut() {
        rx.close();
    }

    let mut iter = block_on_stream(stream);

    assert!(a_tx.is_canceled());
    assert!(b_tx.is_canceled());
    assert!(c_tx.is_canceled());

    assert_eq!(iter.next(), Some(Err(futures::channel::oneshot::Canceled)));
    assert_eq!(iter.next(), Some(Err(futures::channel::oneshot::Canceled)));
    assert_eq!(iter.next(), Some(Err(futures::channel::oneshot::Canceled)));
    assert_eq!(iter.next(), None);
}

#[cfg(feature = "alloc")]
#[test_case]
fn iter_mut_len() {
    use futures::future;
    use futures::stream::FuturesUnordered;
    use std::prelude::v1::*;
    let mut stream = vec![
        future::pending::<()>(),
        future::pending::<()>(),
        future::pending::<()>(),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    let mut iter_mut = stream.iter_mut();
    assert_eq!(iter_mut.len(), 3);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 2);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 1);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 0);
    assert!(iter_mut.next().is_none());
}

#[cfg(feature = "executor")]
#[test_case]
fn iter_cancel() {
    use std::marker::Unpin;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, Ordering};

    use futures::executor::block_on_stream;
    use futures::future::{self, Future, FutureExt};
    use futures::stream::FuturesUnordered;
    use futures::task::{Context, Poll};
    use std::prelude::v1::*;
    struct AtomicCancel<F> {
        future: F,
        cancel: AtomicBool,
    }

    impl<F: Future + Unpin> Future for AtomicCancel<F> {
        type Output = Option<<F as Future>::Output>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.cancel.load(Ordering::Relaxed) {
                Poll::Ready(None)
            } else {
                self.future.poll_unpin(cx).map(Some)
            }
        }
    }

    impl<F: Future + Unpin> AtomicCancel<F> {
        fn new(future: F) -> Self {
            Self { future, cancel: AtomicBool::new(false) }
        }
    }

    let stream = vec![
        AtomicCancel::new(future::pending::<()>()),
        AtomicCancel::new(future::pending::<()>()),
        AtomicCancel::new(future::pending::<()>()),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    for f in stream.iter() {
        f.cancel.store(true, Ordering::Relaxed);
    }

    let mut iter = block_on_stream(stream);

    assert_eq!(iter.next(), Some(None));
    assert_eq!(iter.next(), Some(None));
    assert_eq!(iter.next(), Some(None));
    assert_eq!(iter.next(), None);
}

#[cfg(feature = "alloc")]
#[test_case]
fn iter_len() {
    use futures::future;
    use futures::stream::FuturesUnordered;
    use std::prelude::v1::*;
    let stream = vec![
        future::pending::<()>(),
        future::pending::<()>(),
        future::pending::<()>(),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    let mut iter = stream.iter();
    assert_eq!(iter.len(), 3);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 2);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 1);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
}

#[cfg(feature = "alloc")]
#[test_case]
fn futures_not_moved_after_poll() {
    use futures::future;
    use futures::stream::FuturesUnordered;
    use futures_test::future::FutureTestExt;
    use futures_test::{assert_stream_done, assert_stream_next};
    use std::prelude::v1::*;
    // Future that will be ready after being polled twice,
    // asserting that it does not move.
    let fut = future::ready(()).pending_once().assert_unmoved();
    let mut stream = vec![fut; 3].into_iter().collect::<FuturesUnordered<_>>();
    assert_stream_next!(stream, ());
    assert_stream_next!(stream, ());
    assert_stream_next!(stream, ());
    assert_stream_done!(stream);
}

#[cfg(feature = "alloc")]
#[test_case]
fn len_valid_during_out_of_order_completion() {
    use futures::channel::oneshot;
    use futures::stream::{FuturesUnordered, StreamExt};
    use futures::task::Poll;
    use futures_test::task::noop_context;

    // Complete futures out-of-order and add new futures afterwards to ensure
    // length values remain correct.
    let (a_tx, a_rx) = oneshot::channel::<i32>();
    let (b_tx, b_rx) = oneshot::channel::<i32>();
    let (c_tx, c_rx) = oneshot::channel::<i32>();
    let (d_tx, d_rx) = oneshot::channel::<i32>();

    let mut cx = noop_context();
    let mut stream = FuturesUnordered::new();
    assert_eq!(stream.len(), 0);

    stream.push(a_rx);
    assert_eq!(stream.len(), 1);
    stream.push(b_rx);
    assert_eq!(stream.len(), 2);
    stream.push(c_rx);
    assert_eq!(stream.len(), 3);

    b_tx.send(4).unwrap();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(4))));
    assert_eq!(stream.len(), 2);

    stream.push(d_rx);
    assert_eq!(stream.len(), 3);

    c_tx.send(5).unwrap();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(5))));
    assert_eq!(stream.len(), 2);

    d_tx.send(6).unwrap();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(6))));
    assert_eq!(stream.len(), 1);

    a_tx.send(7).unwrap();
    assert_eq!(stream.poll_next_unpin(&mut cx), Poll::Ready(Some(Ok(7))));
    assert_eq!(stream.len(), 0);
}
