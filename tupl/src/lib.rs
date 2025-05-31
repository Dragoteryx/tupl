#![allow(async_fn_in_trait, non_snake_case, clippy::unused_unit)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]

use core::iter::Chain;
use core::iter::{Empty, empty};
use core::iter::{Once, once};

/// Function related traits.
pub mod fns;
use fns::*;

// Sealed trait.
mod seal {
	pub trait Sealed {}
}

/// Returns `true` if the tuple is the unit tuple.
///
/// # Examples
///
/// ```
/// # use tupl::is_unit;
/// assert!(is_unit::<()>());
/// assert!(!is_unit::<(i32, i32)>());
/// ```
#[inline]
pub const fn is_unit<T: Tuple>() -> bool {
	T::ARITY == 0
}

/// Get the type at a given index of tuple `T`.
pub type TupleIndex<T, const INDEX: usize> = <T as IndexableTuple<INDEX>>::Value;

/// Tuples of unknown size. Implemented for tuples of arity 0 to 32.
pub trait DynTuple: seal::Sealed {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::DynTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(3, tuple.arity());
	/// ```
	fn arity(&self) -> usize;

	/// Returns `true` if this tuple is the unit tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::DynTuple;
	/// let tuple = ();
	/// assert!(tuple.is_unit());
	///
	/// let tuple = (1, 2);
	/// assert!(!tuple.is_unit());
	/// ```
	#[inline]
	fn is_unit(&self) -> bool {
		self.arity() == 0
	}
}

/// Tuples with a known size. Implemented for sized tuples of arity 0 to 32.
pub trait Tuple: DynTuple + Sized {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	const ARITY: usize;
}

/// Tuples that can be converted into an iterator of T. Implemented for sized tuples of arity 0 to 32.
pub trait TupleInto<T>: Tuple {
	type Iterator: Iterator<Item = T>;

	fn tuple_into(self) -> Self::Iterator;
}

/// Tuples that can be joined together. Implemented for sized tuples of arity 0 to 32.
pub trait JoinableTuple<T: JoinableTuple<Self>>: Tuple {
	/// This tuple joined with another tuple.
	type Join: Tuple;

	/// Joins this tuple with another tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::JoinableTuple;
	/// let tuple = (1, 2);
	/// let other = (3, 4, 5);
	/// let joined = tuple.join(other);
	/// assert_eq!((1, 2, 3, 4, 5), joined);
	/// ```
	fn join(self, other: T) -> Self::Join;
}

/// Tuples that can grow. Implemented for sized tuples of arity 0 to 31.
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
	/// assert_eq!((1, 2, 3), tuple);
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
	/// assert_eq!((1, 2, 3), tuple);
	/// ```
	fn prepend<T>(self, value: T) -> Self::Prepend<T>;
}

/// Tuples that are not empty. Implemented for sized tuples of arity 1 to 32.
pub trait NonEmptyTuple: Tuple {
	/// The first element of this tuple.
	type Head;

	/// The last element of this tuple.
	type Tail;

	/// This tuple with its head truncated.
	type TruncateHead: GrowableTuple<Prepend<Self::Head> = Self>;

	/// This tuple with its tail truncated.
	type TruncateTail: GrowableTuple<Append<Self::Tail> = Self>;

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
	/// let (head, tuple) = tuple.truncate_head();
	/// assert_eq!((1, (2, 3)), (head, tuple));
	/// ```
	fn truncate_head(self) -> (Self::Head, Self::TruncateHead);

	/// Consumes this tuple and truncates its tail from the remaining elements.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// let (tuple, tail) = tuple.truncate_tail();
	/// assert_eq!(((1, 2), 3), (tuple, tail));
	/// ```
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

/// Tuples that not unary nor empty. Implemented for sized tuples of arity 2 to 32.
pub trait NonUnaryTuple:
	NonEmptyTuple<
		TruncateHead: NonEmptyTuple<TruncateTail = Self::TruncateHeadTail, Tail = Self::Tail>,
		TruncateTail: NonEmptyTuple<Head = Self::Head, TruncateHead = Self::TruncateHeadTail>,
	>
{
	/// This tuple with its head and tail truncated.
	type TruncateHeadTail: GrowableTuple<
			Prepend<Self::Head> = Self::TruncateTail,
			Append<Self::Tail> = Self::TruncateHead,
		>;

	/// Returns a reference to the head and tail of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonUnaryTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tail) = tuple.head_tail();
	/// assert_eq!((&1, &4), (head, tail));
	/// ```
	fn head_tail(&self) -> (&Self::Head, &Self::Tail);

	/// Returns a mutable reference to the head and tail of this tuple.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonUnaryTuple;
	/// let mut tuple = (1, 2, 3, 4);
	/// let (head, tail) = tuple.head_tail_mut();
	/// assert_eq!((&mut 1, &mut 4), (head, tail));
	/// ```
	fn head_tail_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail);

	/// Consumes this tuple and truncates its head and tail from the remaining elements.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::NonUnaryTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tuple, tail) = tuple.truncate_head_tail();
	/// assert_eq!((1, (2, 3), 4), (head, tuple, tail));
	/// ```
	fn truncate_head_tail(self) -> (Self::Head, Self::TruncateHeadTail, Self::Tail);
}

/// Tuples that can be indexed. Implemented for sized tuples of arity 1 to 32.
pub trait IndexableTuple<const INDEX: usize>: NonEmptyTuple {
	/// The type of the value at the given index.
	type Value;

	/// Returns a reference to the value at the given index.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::IndexableTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&2, IndexableTuple::<1>::index_ref(&tuple));
	/// ```
	fn index_ref(&self) -> &Self::Value;

	/// Returns a mutable reference to the value at the given index.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::IndexableTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 2, IndexableTuple::<1>::index_mut(&mut tuple));
	/// ```
	fn index_mut(&mut self) -> &mut Self::Value;

	/// Consumes this tuple and returns the value at the given index.
	///
	/// # Examples
	///
	/// ```
	/// # use tupl::IndexableTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(2, IndexableTuple::<1>::into_index(tuple));
	/// ```
	fn into_index(self) -> Self::Value;
}

// Implements all relevant traits.
tupl_macros::impl_traits!();

// The base case of the TupleInto trait simply returns an empty iterator.
impl<T> TupleInto<T> for () {
	type Iterator = Empty<T>;

	#[inline]
	fn tuple_into(self) -> Self::Iterator {
		empty()
	}
}

// Blanket implementation of the TupleInto trait for tuples of non-zero arity.
impl<T, U: NonEmptyTuple<TruncateTail: TupleInto<T>, Tail: Into<T>>> TupleInto<T> for U {
	type Iterator = Chain<<U::TruncateTail as TupleInto<T>>::Iterator, Once<T>>;

	fn tuple_into(self) -> Self::Iterator {
		let (tuple, tail) = self.truncate_tail();
		tuple.tuple_into().chain(once(tail.into()))
	}
}
