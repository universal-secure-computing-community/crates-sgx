use std::string::ToString;
use crates_unittest::test_case;
#[cfg(feature = "std")]
mod mock_writer {
    use std::prelude::v1::*;
    use futures::io::AsyncWrite;
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    pub struct MockWriter {
        fun: Box<dyn FnMut(&[u8]) -> Poll<io::Result<usize>>>,
    }

    impl MockWriter {
        pub fn new(fun: impl FnMut(&[u8]) -> Poll<io::Result<usize>> + 'static) -> Self {
            MockWriter { fun: Box::new(fun) }
        }
    }

    impl AsyncWrite for MockWriter {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            (self.get_mut().fun)(buf)
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            panic!()
        }

        fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            panic!()
        }
    }
}

/// Verifies that the default implementation of `poll_write_vectored`
/// calls `poll_write` with an empty slice if no buffers are provided.
#[cfg(feature = "std")]
#[test_case]
fn write_vectored_no_buffers() {
    use futures::io::AsyncWrite;
    use futures_test::task::panic_context;
    use std::io;
    use std::pin::Pin;
    use std::task::Poll;

    use mock_writer::MockWriter;

    let mut writer = MockWriter::new(|buf| {
        assert_eq!(buf, b"");
        Err(io::ErrorKind::BrokenPipe.into()).into()
    });
    let cx = &mut panic_context();
    let bufs = &mut [];

    let res = Pin::new(&mut writer).poll_write_vectored(cx, bufs);
    let res = res.map_err(|e| e.kind());
    assert_eq!(res, Poll::Ready(Err(io::ErrorKind::BrokenPipe)))
}

/// Verifies that the default implementation of `poll_write_vectored`
/// calls `poll_write` with the first non-empty buffer.
#[cfg(feature = "std")]
#[test_case]
fn write_vectored_first_non_empty() {
    use futures::io::AsyncWrite;
    use futures_test::task::panic_context;
    use std::io;
    use std::pin::Pin;
    use std::task::Poll;

    use mock_writer::MockWriter;

    let mut writer = MockWriter::new(|buf| {
        assert_eq!(buf, b"four");
        Poll::Ready(Ok(4))
    });
    let cx = &mut panic_context();
    let bufs = &mut [
        io::IoSlice::new(&[]), 
        io::IoSlice::new(&[]),
        io::IoSlice::new(b"four")
    ];

    let res = Pin::new(&mut writer).poll_write_vectored(cx, bufs);
    let res = res.map_err(|e| e.kind());
    assert_eq!(res, Poll::Ready(Ok(4)));
}

