use super::*;

/// Functions that are called by value and return a value that can be turned into a [`Future`].
pub trait FnFutureOnce<T: Tuple>: FnOnce<T, Output: IntoFuture<IntoFuture = Self::Future>> {
	/// The [`Future`] returned by this function.
	type Future: Future<Output = Self::FutOutput>;
	/// The output of the [`Future`] returned by this function.
	type FutOutput;
}

impl<T: Tuple, F: FnOnce<T, Output: IntoFuture>> FnFutureOnce<T> for F {
	type Future = <F::Output as IntoFuture>::IntoFuture;
	type FutOutput = <F::Output as IntoFuture>::Output;
}

/// Functions that can be called by mutable reference and return a value that can be turned into a [`Future`].
pub trait FnFutureMut<T: Tuple>: FnMut<T> + FnFutureOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnFutureOnce<T>> FnFutureMut<T> for F {}

/// Functions that can be called by reference and return a value that can be turned into a [`Future`].
pub trait FnFuture<T: Tuple>: Fn<T> + FnFutureMut<T> {}
impl<T: Tuple, F: Fn<T> + FnFutureMut<T>> FnFuture<T> for F {}
