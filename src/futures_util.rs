use core::{
    future::{self, IntoFuture},
    marker::PhantomData,
};

use crate::cancellation::{TrMayCancel, TrCancellationToken};

pub const fn make_ready<T>(t: T) -> MakeReady<T> {
    MakeReady::new(t)
}

pub const fn make_pending<T>() -> MakePending<T> {
    MakePending::new()
}

pub struct MakeReady<T>(T);

pub struct MakePending<T>(PhantomData<T>);

// ---------------------------------------------------------
// MakeReady<T> impl IntoFuture, Ready<T> impl TrMayCancel
// ---------------------------------------------------------

impl<T> MakeReady<T> {
    pub const fn new(t: T) -> MakeReady<T> {
        MakeReady(t)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn into_future(self) -> future::Ready<T> {
        future::ready(self.into_inner())
    }

    pub fn may_cancel_with<'f, C: TrCancellationToken>(self, _: &'f mut C) -> future::Ready<T> {
        self.into_future()
    }
}

impl<T> IntoFuture for MakeReady<T> {
    type IntoFuture = future::Ready<T>;
    type Output = T;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        MakeReady::into_future(self)
    }
}

impl<'a, T> TrMayCancel<'a> for MakeReady<T>
where
    Self: 'a + IntoFuture,
{
    type MayCancelOutput = T;

    #[inline]
    fn may_cancel_with<'f, C: TrCancellationToken>(
        self,
        _: &'f mut C,
    ) -> impl IntoFuture<Output = Self::MayCancelOutput>
    where
        Self: 'f
    {
        self.into_future()
    }
}

// ---------------------------------------------------------
// MakePending impl IntoFuture, TrMayCancel
// ---------------------------------------------------------

impl<T> MakePending<T> {
    pub const fn new() -> MakePending<T> {
        MakePending(PhantomData)
    }

    pub fn into_future(self) -> future::Pending<T> {
        future::pending()
    }
}

impl<T> IntoFuture for MakePending<T> {
    type IntoFuture = future::Pending<T>;
    type Output = T;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        MakePending::into_future(self)
    }
}

// ---------------------------------------------------------
// MakePending impl IntoFuture, TrMayCancel
// ---------------------------------------------------------

impl<'a, T> TrMayCancel<'a> for future::Ready<T>
where
    Self: 'a + IntoFuture,
    T: 'a,
{
    type MayCancelOutput = <future::Ready<T> as Future>::Output;

    fn may_cancel_with<'f, C: TrCancellationToken>(
        self,
        _: &'f mut C,
    ) -> impl IntoFuture<Output = Self::MayCancelOutput>
    where
        Self: 'f
    {
        MakeReady::new(self.into_inner())
    }
}

#[cfg(test)]
mod tests_ {
    use super::*;
    use crate::NonCancellableToken;

    #[compio::test]
    async fn make_ready() {
        const ANSWER: usize = 1usize;
        let f = MakeReady::new(ANSWER);
        let x = f.may_cancel_with(NonCancellableToken::shared_mut()).await;
        assert_eq!(x, ANSWER)
    }
}
