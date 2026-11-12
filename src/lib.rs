//! Raw OASIS PKCS#11 3.2 declarations. See the crate README and NOTICE.txt.
//!
//! This layer supplies C layouts and nullable unsafe function pointers. The caller
//! owns module lifetime, version negotiation, buffer validity and synchronization.
//! A declared mechanism or function is not a capability or security-policy claim.
#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![doc = include_str!("../HEADER-NOTES.md")]

#[cfg(all(
    target_os = "linux",
    target_vendor = "unknown",
    target_env = "gnu",
    target_arch = "x86_64",
    target_pointer_width = "64",
    target_endian = "little"
))]
#[path = "x86_64-unknown-linux-gnu/mod.rs"]
mod platform;

#[cfg(all(
    target_os = "windows",
    target_vendor = "pc",
    target_env = "msvc",
    target_arch = "x86_64",
    target_pointer_width = "64",
    target_endian = "little"
))]
#[path = "x86_64-pc-windows-msvc/mod.rs"]
mod platform;

#[cfg(any(
    all(
        target_os = "linux",
        target_vendor = "unknown",
        target_env = "gnu",
        target_arch = "x86_64",
        target_pointer_width = "64",
        target_endian = "little"
    ),
    all(
        target_os = "windows",
        target_vendor = "pc",
        target_env = "msvc",
        target_arch = "x86_64",
        target_pointer_width = "64",
        target_endian = "little"
    )
))]
pub use platform::*;

#[cfg(not(any(
    all(
        target_os = "linux",
        target_vendor = "unknown",
        target_env = "gnu",
        target_arch = "x86_64",
        target_pointer_width = "64",
        target_endian = "little"
    ),
    all(
        target_os = "windows",
        target_vendor = "pc",
        target_env = "msvc",
        target_arch = "x86_64",
        target_pointer_width = "64",
        target_endian = "little"
    )
)))]
compile_error!("Fe11-sys supports only x86_64-unknown-linux-gnu and x86_64-pc-windows-msvc.");
