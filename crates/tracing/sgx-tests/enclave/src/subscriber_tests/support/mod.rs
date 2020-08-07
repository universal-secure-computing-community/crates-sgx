#![allow(dead_code)]
pub mod event;
pub mod field;
mod metadata;
pub mod span;
pub mod subscriber;
use std::prelude::v1::*;
#[derive(Debug, Eq, PartialEq)]
pub(in crate::subscriber_tests::support) enum Parent {
    ContextualRoot,
    Contextual(String),
    ExplicitRoot,
    Explicit(String),
}
