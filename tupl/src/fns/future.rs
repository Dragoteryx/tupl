use super::*;
use core::future::Future;

/// A function that is called by value and returns a [`Future`](core::future::Future).
pub trait FnFutureOnce<T: Tuple>: FnOnce<T, Output = Self::Future> {
	/// The type of the [`Future`](core::future::Future) returned by this function.
	type Future: Future<Output = Self::FutOutput>;
	/// The type of the value returned by the [`Future`](core::future::Future) returned by this function.
	type FutOutput;
}

impl<T: Tuple, F: FnOnce<T, Output = Fut>, Fut: Future> FnFutureOnce<T> for F {
	type Future = Fut;
	type FutOutput = Fut::Output;
}

/// A function that is called by mutable reference and returns a [`Future`](core::future::Future).
pub trait FnFutureMut<T: Tuple>: FnMut<T> + FnFutureOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnFutureOnce<T>> FnFutureMut<T> for F {}

/// A function that is called by reference and returns a [`Future`](core::future::Future).
pub trait FnFuture<T: Tuple>: Fn<T> + FnFutureMut<T> {}
impl<T: Tuple, F: Fn<T> + FnFutureMut<T>> FnFuture<T> for F {}
