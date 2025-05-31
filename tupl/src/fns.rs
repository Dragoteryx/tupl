use super::*;
use futures::Stream;

/// Functions that return an [`Iterator`].
pub mod iter;

/// Functions that return an [`Option`].
pub mod option;

/// Functions that return a [`Result`].
pub mod result;

/// Functions that return a [`Stream`].
pub mod stream;

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

/// Async functions that are called by value.
pub trait AsyncFnOnce<T: Tuple> {
	/// The return type of this async function.
	type Output;

	/// Call the async function by value.
	async fn async_call_once(self, args: T) -> Self::Output;
}

/// Async functions that are called by mutable reference.
pub trait AsyncFnMut<T: Tuple>: AsyncFnOnce<T> {
	/// Call the async function by mutable reference.
	async fn async_call_mut(&mut self, args: T) -> Self::Output;
}

/// Async functions that are called by reference.
pub trait AsyncFn<T: Tuple>: AsyncFnMut<T> {
	/// Call the async function by reference.
	async fn async_call(&self, args: T) -> Self::Output;
}
