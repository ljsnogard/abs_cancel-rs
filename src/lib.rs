#![feature(try_trait_v2)]

#![no_std]

#[cfg(test)]
extern crate std;

mod cancellation;

pub mod futures_util;

pub use cancellation::{
    CancelledToken, NonCancellableToken,
    TrCancellationToken, TrMayCancel,
};
pub use futures_util::{
    make_pending, make_ready,
};
