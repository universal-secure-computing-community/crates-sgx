use sct::Error;
use std::prelude::v1::*;
use crates_unittest::test_case;

#[test_case]
fn test_unknown_log_is_not_fatal() {
    assert_eq!(false, Error::UnknownLog.should_be_fatal());
}

#[test_case]
fn test_unknown_sct_version_is_not_fatal() {
    assert_eq!(false, Error::UnsupportedSCTVersion.should_be_fatal());
}

#[test_case]
fn test_other_errors_are_fatal() {
    assert_eq!(true, Error::MalformedSCT.should_be_fatal());
    assert_eq!(true, Error::InvalidSignature.should_be_fatal());
    assert_eq!(true, Error::TimestampInFuture.should_be_fatal());
}
