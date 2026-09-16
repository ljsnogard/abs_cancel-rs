#![no_std]

mod cancellation;

pub use cancellation::{
    CancelledToken, NonCancellableToken,
    TrCancellationToken, TrMayCancel,
};
