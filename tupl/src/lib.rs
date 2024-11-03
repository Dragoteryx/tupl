#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(non_snake_case, clippy::unused_unit)]
#![forbid(unsafe_code)]
#![no_std]

//! A `#![no_std]` crate for handling Rust tuples using traits.

/// Function related traits.
pub mod fns;
use fns::*;

// Sealed trait.
mod seal {
	pub trait Sealed {}
}

/// Get the type at a given index of tuple `T`.
pub type TupleIndex<T, const INDEX: usize> = <T as IndexableTuple<INDEX>>::Value;

/// Tuples of unknown size. Implemented for tuples of arity 0 to 32.
pub trait DynTuple: seal::Sealed {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	fn arity(&self) -> usize;
}

/// Tuples with a known size. Implemented for sized tuples of arity 0 to 32.
pub trait Tuple: DynTuple + Sized {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	const ARITY: usize;
}

/// Tuples that can be joined together. Implemented for tuples of arity 0 to 32.
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
	/// let other = (3, 4);
	/// let joined = tuple.join(other);
	/// assert_eq!((1, 2, 3, 4), joined);
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
pub trait NonUnaryTuple: NonEmptyTuple<TruncateHead: NonEmptyTuple<Tail = Self::Tail>, TruncateTail: NonEmptyTuple<Head = Self::Head>> {
	/// This tuple with its head and tail truncated.
	type TruncateHeadTail: GrowableTuple<Prepend<Self::Head> = Self::TruncateTail, Append<Self::Tail> = Self::TruncateHead>;

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
	/// assert_eq!(&2, IndexableTuple::<1>::get(&tuple));
	/// ```
	fn get(&self) -> &Self::Value;

	/// Returns a mutable reference to the value at the given index.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::IndexableTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 2, IndexableTuple::<1>::get_mut(&mut tuple));
	/// ```
	fn get_mut(&mut self) -> &mut Self::Value;

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

// Implements all traits.
tupl_macros::impl_traits!();
