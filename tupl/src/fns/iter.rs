use super::*;

/// A function that is called by value and returns an [`Iterator`](core::iter::Iterator).
pub trait FnIteratorOnce<T: Tuple>: FnOnce<T, Output = Self::Iterator> {
	/// The type of the [`Iterator`](core::iter::Iterator) returned by this function.
	type Iterator: Iterator<Item = Self::Item>;
	/// The type of the items yielded by the [`Iterator`](core::iter::Iterator) returned by this function.
	type Item;
}

impl<T: Tuple, F: FnOnce<T, Output = Iter>, Iter: Iterator> FnIteratorOnce<T> for F {
	type Iterator = Iter;
	type Item = Iter::Item;
}

/// A function that is called by mutable reference and returns an [`Iterator`](core::iter::Iterator).
pub trait FnIteratorMut<T: Tuple>: FnMut<T> + FnIteratorOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnIteratorOnce<T>> FnIteratorMut<T> for F {}

/// A function that is called by reference and returns an [`Iterator`](core::iter::Iterator).
pub trait FnIterator<T: Tuple>: Fn<T> + FnIteratorMut<T> {}
impl<T: Tuple, F: Fn<T> + FnIteratorMut<T>> FnIterator<T> for F {}
