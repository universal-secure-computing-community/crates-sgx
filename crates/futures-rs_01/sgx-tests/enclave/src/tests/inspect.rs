

use futures::prelude::*;
use futures::future::{ok, err};

use std::prelude::v1::*;
use crates_unittest::test_case;


#[test_case]
fn smoke() {
    let mut counter = 0;

    {
        let work = ok::<u32, u32>(40).inspect(|val| { counter += *val; });
        assert_eq!(work.wait(), Ok(40));
    }

    assert_eq!(counter, 40);

    {
        let work = err::<u32, u32>(4).inspect(|val| { counter += *val; });
        assert_eq!(work.wait(), Err(4));
    }

    assert_eq!(counter, 40);
}
