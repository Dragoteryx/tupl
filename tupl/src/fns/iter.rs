use super::*;

/// Functions that are called by value and return a value that can be turned into a [`Iterator`].
pub trait FnIteratorOnce<T: Tuple>:
	FnOnce<T, Output: IntoIterator<IntoIter = Self::Iterator>>
{
	/// The [`Iterator`] returned by this function.
	type Iterator: Iterator<Item = Self::Item>;
	/// The item yielded by the [`Iterator`] returned by this function.
	type Item;
}

impl<T: Tuple, F: FnOnce<T, Output: IntoIterator>> FnIteratorOnce<T> for F {
	type Iterator = <F::Output as IntoIterator>::IntoIter;
	type Item = <F::Output as IntoIterator>::Item;
}

/// Functions that can be called by mutable reference and return a value that can be turned into a [`Iterator`].
pub trait FnIteratorMut<T: Tuple>: FnMut<T> + FnIteratorOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnIteratorOnce<T>> FnIteratorMut<T> for F {}

/// Functions that can be called by reference and return a value that can be turned into a [`Iterator`].
pub trait FnIterator<T: Tuple>: Fn<T> + FnIteratorMut<T> {}
impl<T: Tuple, F: Fn<T> + FnIteratorMut<T>> FnIterator<T> for F {}
