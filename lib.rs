#![allow(dead_code)]
extern crate rustc_serialize as serialize;

#[path = "rust-crypto/src/buffer.rs"]
pub mod buffer;
#[path = "rust-crypto/src/digest.rs"]
pub mod digest;
#[path = "rust-crypto/src/sha2.rs"]
pub mod sha2;
#[path = "rust-crypto/src/symmetriccipher.rs"]
pub mod symmetriccipher;

#[path = "rust-crypto/src/cryptoutil.rs"]
mod cryptoutil;
#[path = "rust-crypto/src/simd.rs"]
mod simd;

pub use sha2::*;
