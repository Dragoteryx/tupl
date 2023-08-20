use super::*;

/// Functions that return a [`Future`](core::future::Future).
pub mod future;

/// Functions that return an [`Iterator`](core::iter::Iterator).
pub mod iter;

/// Functions that return an [`Option`](core::option::Option).
pub mod option;

/// Functions that return a [`Result`](core::result::Result).
pub mod result;

/// A function that is called by value.
pub trait FnOnce<T: Tuple> {
	/// The return type of this function.
	type Output;

	/// Call the function by value.
	fn call_once(self, args: T) -> Self::Output;
}

/// A function that is called by mutable reference.
pub trait FnMut<T: Tuple>: FnOnce<T> {
	/// Call the function by mutable reference.
	fn call_mut(&mut self, args: T) -> Self::Output;
}

/// A function that is called by reference.
pub trait Fn<T: Tuple>: FnOnce<T> {
	/// Call the function by reference.
	fn call(&self, args: T) -> Self::Output;
}
