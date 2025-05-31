use super::*;

/// Functions that are called by value and return a [`Stream`].
pub trait FnStreamOnce<T: Tuple>: FnOnce<T, Output: Stream<Item = Self::Item>> {
	/// The item yielded by the [`Stream`] returned by this function.
	type Item;
}

impl<T: Tuple, F: FnOnce<T, Output: Stream>> FnStreamOnce<T> for F {
	type Item = <F::Output as Stream>::Item;
}

/// Functions that can be called by mutable reference and return a [`Stream`].
pub trait FnStreamMut<T: Tuple>: FnMut<T> + FnStreamOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnStreamOnce<T>> FnStreamMut<T> for F {}

/// Functions that can be called by reference and return a [`Stream`].
pub trait FnStream<T: Tuple>: Fn<T> + FnStreamMut<T> {}
impl<T: Tuple, F: Fn<T> + FnStreamMut<T>> FnStream<T> for F {}

/// Async functions that are called by value and return a [`Stream`].
pub trait AsyncFnStreamOnce<T: Tuple>: AsyncFnOnce<T, Output: Stream<Item = Self::Item>> {
	/// The item yielded by the [`Stream`] returned by this async function.
	type Item;
}

impl<T: Tuple, F: AsyncFnOnce<T, Output: Stream>> AsyncFnStreamOnce<T> for F {
	type Item = <F::Output as Stream>::Item;
}

/// Async functions that can be called by mutable reference and return a [`Stream`].
pub trait AsyncFnStreamMut<T: Tuple>: AsyncFnMut<T> + AsyncFnStreamOnce<T> {}
impl<T: Tuple, F: AsyncFnMut<T> + AsyncFnStreamOnce<T>> AsyncFnStreamMut<T> for F {}

/// Async functions that can be called by reference and return a [`Stream`].
pub trait AsyncFnStream<T: Tuple>: AsyncFn<T> + AsyncFnStreamMut<T> {}
impl<T: Tuple, F: AsyncFn<T> + AsyncFnStreamMut<T>> AsyncFnStream<T> for F {}
