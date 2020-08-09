/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

#![no_std]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    /// A future polling two other futures and returning the output of the first one to complete.
    ///
    /// ## Example
    ///
    /// ```
    /// use futures_race::{Race, RaceExt};
    /// # use smol::Timer;
    /// # use std::time::Duration;
    ///
    /// # smol::run(async {
    /// let foo = async {
    ///     Timer::new(Duration::from_millis(100)).await;
    ///     42
    /// };
    ///
    /// let bar = async {
    ///     Timer::new(Duration::from_millis(250)).await;
    ///     24
    /// };
    ///
    /// let foobar = foo.race(bar).await;
    /// assert_eq!(foobar, 42);
    /// # })
    /// ```
    pub struct Race<Left, Right>
    where
        Left: Future,
        Right: Future<Output = Left::Output>,
    {
        #[pin]
        left: Left,
        #[pin]
        right: Right,
    }
}

/// An extension trait for [`Future`]s that provides a way to create [`Race`]s.
pub trait RaceExt: Future {
    /// Given a second future with the same output, creates and returns a new [`Race`] that will
    /// poll both futures and return the output of the first one to complete.
    ///
    /// ## Example
    ///
    /// ```
    /// use futures_race::{Race, RaceExt};
    /// # use smol::Timer;
    /// # use std::time::Duration;
    ///
    /// # smol::run(async {
    /// let foo = async {
    ///     Timer::new(Duration::from_millis(100)).await;
    ///     42
    /// };
    ///
    /// let bar = async {
    ///     Timer::new(Duration::from_millis(250)).await;
    ///     24
    /// };
    ///
    /// let foobar = foo.race(bar).await;
    /// assert_eq!(foobar, 42);
    /// # })
    /// ```
    fn race<With>(self, with: With) -> Race<Self, With>
    where
        Self: Sized,
        With: Future<Output = Self::Output>,
    {
        Race {
            left: self,
            right: with,
        }
    }
}

impl<Fut: Future> RaceExt for Fut {}

impl<Left, Right> Future for Race<Left, Right>
where
    Left: Future,
    Right: Future<Output = Left::Output>,
{
    type Output = Left::Output;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        if let poll @ Poll::Ready(_) = this.left.poll(ctx) {
            return poll;
        }

        if let poll @ Poll::Ready(_) = this.right.poll(ctx) {
            return poll;
        }

        Poll::Pending
    }
}
