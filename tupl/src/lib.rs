#![no_std]

//! A `#![no_std]` crate for handling Rust tuples using traits.

/// Function related traits.
pub mod fns;

/// Whether or not this tuple type is the empty tuple.
/// # Examples
/// ```
/// # use tupl::is_unit;
/// assert!(is_unit::<()>());
/// assert!(!is_unit::<(i32,)>());
/// ```
pub const fn is_unit<T: Tuple>() -> bool {
	T::ARITY == 0
}

// Sealed trait.
mod private {
	pub trait Sealed {}
}

/// A **sealed** trait representing tuples. It is implemented for tuples of arity 0 to 50.
pub trait Tuple: private::Sealed {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	const ARITY: usize;
}

/// A trait representing tuples that can grow. It is implemented for tuples of arity 0 to 49.
pub trait GrowableTuple: Tuple {
	/// This tuple with an extra element `T` appended to it.
	type Append<T>: NonEmptyTuple<TruncateTail = Self, Tail = T>;

	/// This tuple with an extra element `T` prepended to it.
	type Prepend<T>: NonEmptyTuple<Head = T, TruncateHead = Self>;

	/// Consumes this tuple and appends a value to it, returning a new tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::GrowableTuple;
	/// let tuple = (1, 2);
	/// let tuple = tuple.append(3);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	fn append<T>(self, value: T) -> Self::Append<T>;

	/// Consumes this tuple and prepends a value to it, returning a new tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::GrowableTuple;
	/// let tuple = (2, 3);
	/// let tuple = tuple.prepend(1);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	fn prepend<T>(self, value: T) -> Self::Prepend<T>;
}

/// A trait representing tuples that are not empty. It is implemented for tuples of arity 1 to 50.
pub trait NonEmptyTuple: Tuple {
	/// The first element of this tuple.
	type Head;

	/// The last element of this tuple.
	type Tail;

	/// This tuple with its head truncated.
	type TruncateHead: GrowableTuple;

	/// This tuple with its tail truncated.
	type TruncateTail: GrowableTuple;

	/// Returns a reference to the head of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&1, tuple.head());
	/// ```
	fn head(&self) -> &Self::Head;

	/// Returns a mutable reference to the head of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 1, tuple.head_mut());
	/// ```
	fn head_mut(&mut self) -> &mut Self::Head;

	/// Returns a reference to the tail of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&3, tuple.tail());
	/// ```
	fn tail(&self) -> &Self::Tail;

	/// Returns a mutable reference to the tail of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 3, tuple.tail_mut());
	/// ```
	fn tail_mut(&mut self) -> &mut Self::Tail;

	/// Consumes this tuple and truncates its head from the remaining elements.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_head();
	/// assert_eq!(tuple, (1, (2, 3)));
	/// ```
	fn truncate_head(self) -> (Self::Head, Self::TruncateHead);

	/// Consumes this tuple and truncates its tail from the remaining elements.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_tail();
	/// assert_eq!(tuple, ((1, 2), 3));
	/// ```
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

// Implements all traits.
use core::ops::Fn as StdFn;
use core::ops::FnMut as StdFnMut;
use core::ops::FnOnce as StdFnOnce;
use fns::{Fn, FnMut, FnOnce};
tupl_macros::impl_traits!();
