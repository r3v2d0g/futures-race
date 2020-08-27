/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

// ======================================== Configuration ======================================= \\

#![no_std]

// ======================================== Documentation ======================================= \\

//! Deprecated in favor of [`futures-micro`](https://github.com/irrustible/futures-micro) and
//! [`futures-lite`](https://github.com/stjepang/futures-lite).

// =========================================== Imports ========================================== \\

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

// ============================================ Types =========================================== \\

pin_project! {
    #[derive(Debug)]
    #[deprecated(since = "1.2.0", note = "please use `futures-micro` or `futures-lite` instead")]
    /// Deprecated in favor of [`futures-micro`](https://github.com/irrustible/futures-micro) and
    /// [`futures-lite`](https://github.com/stjepang/futures-lite).
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

// ========================================= Interfaces ========================================= \\

#[deprecated(since = "1.2.0", note = "please use `futures-micro` or `futures-lite` instead")]
/// Deprecated in favor of [`futures-micro`](https://github.com/irrustible/futures-micro) and
/// [`futures-lite`](https://github.com/stjepang/futures-lite).
pub trait RaceExt: Future {
    #[deprecated(since = "1.2.0", note = "please use `futures-micro` or `futures-lite` instead")]
    /// Deprecated in favor of [`futures-micro`](https://github.com/irrustible/futures-micro) and
    /// [`futures-lite`](https://github.com/stjepang/futures-lite).
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

// ========================================= impl Future ======================================== \\

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
