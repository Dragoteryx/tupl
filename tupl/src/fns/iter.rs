use super::*;

/// Functions that are called by value and return a value that can be turned into a [`Iterator`].
pub trait FnIteratorOnce<T: Tuple>: FnOnce<T, Output: Iterator<Item = Self::Item>> {
	/// The item yielded by the [`Iterator`] returned by this function.
	type Item;
}

impl<T: Tuple, F: FnOnce<T, Output: Iterator>> FnIteratorOnce<T> for F {
	type Item = <F::Output as Iterator>::Item;
}

/// Functions that can be called by mutable reference and return an [`Iterator`].
pub trait FnIteratorMut<T: Tuple>: FnMut<T> + FnIteratorOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnIteratorOnce<T>> FnIteratorMut<T> for F {}

/// Functions that can be called by reference and return an [`Iterator`].
pub trait FnIterator<T: Tuple>: Fn<T> + FnIteratorMut<T> {}
impl<T: Tuple, F: Fn<T> + FnIteratorMut<T>> FnIterator<T> for F {}

/// Async functions that are called by value and return an [`Iterator`].
pub trait AsyncFnIteratorOnce<T: Tuple>:
	AsyncFnOnce<T, Output: Iterator<Item = Self::Item>>
{
	/// The item yielded by the [`Iterator`] returned by this async function.
	type Item;
}

impl<T: Tuple, F: AsyncFnOnce<T, Output: Iterator>> AsyncFnIteratorOnce<T> for F {
	type Item = <F::Output as Iterator>::Item;
}

/// Async functions that can be called by mutable reference and return an [`Iterator`].
pub trait AsyncFnIteratorMut<T: Tuple>: AsyncFnMut<T> + AsyncFnIteratorOnce<T> {}
impl<T: Tuple, F: AsyncFnMut<T> + AsyncFnIteratorOnce<T>> AsyncFnIteratorMut<T> for F {}

/// Async functions that can be called by reference and return an [`Iterator`].
pub trait AsyncFnIterator<T: Tuple>: AsyncFn<T> + AsyncFnIteratorMut<T> {}
impl<T: Tuple, F: AsyncFn<T> + AsyncFnIteratorMut<T>> AsyncFnIterator<T> for F {}
