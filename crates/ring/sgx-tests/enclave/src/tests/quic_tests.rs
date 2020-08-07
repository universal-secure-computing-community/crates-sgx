// Copyright 2018 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#![forbid(
    anonymous_parameters,
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    warnings
)]

use ring::{aead::quic, test};
use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn quic_aes_128() {
    test_quic(&quic::AES_128, test_file!("quic_aes_128_tests.txt"));
}

#[test_case]
fn quic_aes_256() {
    test_quic(&quic::AES_256, test_file!("quic_aes_256_tests.txt"));
}

#[test_case]
fn quic_chacha20() {
    test_quic(&quic::CHACHA20, test_file!("quic_chacha20_tests.txt"));
}

fn test_quic(alg: &'static quic::Algorithm, test_file: test::File) {
    test_sample_len(alg);

    test::run(test_file, |section, test_case| {
        assert_eq!(section, "");
        let key_bytes = test_case.consume_bytes("KEY");
        let sample = test_case.consume_bytes("SAMPLE");
        let mask = test_case.consume_bytes("MASK");

        let key = quic::HeaderProtectionKey::new(alg, &key_bytes)?;

        assert_eq!(mask.as_ref(), key.new_mask(&sample)?);

        Ok(())
    });
}

fn test_sample_len(alg: &'static quic::Algorithm) {
    let key_len = alg.key_len();
    let key_data = vec![0u8; key_len];

    let key = quic::HeaderProtectionKey::new(alg, &key_data).unwrap();

    let sample_len = 16;
    let sample_data = vec![0u8; sample_len + 2];

    // Sample is the right size.
    assert!(key.new_mask(&sample_data[..sample_len]).is_ok());

    // Sample is one byte too small.
    assert!(key.new_mask(&sample_data[..(sample_len - 1)]).is_err());

    // Sample is one byte too big.
    assert!(key.new_mask(&sample_data[..(sample_len + 1)]).is_err());

    // Sample is empty.
    assert!(key.new_mask(&[]).is_err());
}
