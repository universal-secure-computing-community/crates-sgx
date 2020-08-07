use std::string::ToString;
use crates_unittest::test_case;
#[cfg(feature = "executor")] // executor::
#[test_case]
fn ignore_err() {
    use futures::executor::block_on;
    use futures::future::{err, ok, select_ok};

    let v = vec![
        err(1),
        err(2),
        ok(3),
        ok(4),
    ];

    let (i, v) = block_on(select_ok(v)).ok().unwrap();
    assert_eq!(i, 3);

    assert_eq!(v.len(), 1);

    let (i, v) = block_on(select_ok(v)).ok().unwrap();
    assert_eq!(i, 4);

    assert!(v.is_empty());
}

#[cfg(feature = "executor")] // executor::
#[test_case]
fn last_err() {
    use futures::executor::block_on;
    use futures::future::{err, ok, select_ok};

    let v = vec![
        ok(1),
        err(2),
        err(3),
    ];

    let (i, v) = block_on(select_ok(v)).ok().unwrap();
    assert_eq!(i, 1);

    assert_eq!(v.len(), 2);

    let i = block_on(select_ok(v)).err().unwrap();
    assert_eq!(i, 3);
}
