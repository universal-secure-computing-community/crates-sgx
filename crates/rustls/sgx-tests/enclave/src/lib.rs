#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate rustls;
extern crate webpki;

extern crate crates_unittest;
mod tests;


#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    tests::run_tests();
}