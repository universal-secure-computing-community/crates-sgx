#![cfg(unix)]

extern crate sgx_libc as libc;

use super::support::*;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn drop_then_get_a_signal() {
    let mut rt = CurrentThreadRuntime::new().unwrap();
    let signal = run_with_timeout(&mut rt, Signal::new(libc::SIGUSR1))
        .expect("failed to create first signal");
    drop(signal);

    send_signal(libc::SIGUSR1);
    let signal = run_with_timeout(&mut rt, Signal::new(libc::SIGUSR1))
        .expect("failed to create signal")
        .into_future()
        .map(|_| ())
        .map_err(|(e, _)| panic!("{}", e));

    run_with_timeout(&mut rt, signal).expect("failed to get signal");
}
