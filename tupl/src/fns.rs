use super::*;
use core::future::{Future, IntoFuture};

/// Functions that return a [`Future`].
pub mod future;

/// Functions that return an [`Iterator`].
pub mod iter;

/// Functions that return an [`Option`].
pub mod option;

/// Functions that return a [`Result`].
pub mod result;

/// Functions that are called by value.
pub trait FnOnce<T: Tuple> {
	/// The return type of this function.
	type Output;

	/// Call the function by value.
	fn call_once(self, args: T) -> Self::Output;
}

/// Functions that can be called by mutable reference.
pub trait FnMut<T: Tuple>: FnOnce<T> {
	/// Call the function by mutable reference.
	fn call_mut(&mut self, args: T) -> Self::Output;
}

/// Functions that can be called by reference.
pub trait Fn<T: Tuple>: FnMut<T> {
	/// Call the function by reference.
	fn call(&self, args: T) -> Self::Output;
}
