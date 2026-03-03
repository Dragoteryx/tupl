#![allow(async_fn_in_trait, non_snake_case, clippy::unused_unit)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]

/// Function related traits.
pub mod fns;

pub mod util;

// Sealed trait.
mod seal {
	pub trait Sealed: Sized {}
}

/// The base tuple trait. Implemented for tuples of arity 0 to 32.
pub trait Tuple: seal::Sealed {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	const ARITY: usize;

	/// Whether this tuple is the unit tuple or not.
	const UNIT: bool;

	/// A tuple containing references to the elements of this tuple.
	type Ref<'t>: Tuple
	where
		Self: 't;

	/// A tuple containing mutable references to the elements of this tuple.
	type Mut<'t>: Tuple
	where
		Self: 't;

	/// Returns a tuple containing references to the elements of this tuple.
	///
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// let tuple_ref = tuple.as_ref();
	/// assert_eq!(&1, tuple_ref.0);
	/// assert_eq!(&2, tuple_ref.1);
	/// assert_eq!(&3, tuple_ref.2);
	/// ```
	fn as_ref(&self) -> Self::Ref<'_>;

	/// Returns a tuple containing mutable references to the elements of this tuple.
	///
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// let tuple_mut = tuple.as_mut();
	/// assert_eq!(&mut 1, tuple_mut.0);
	/// assert_eq!(&mut 2, tuple_mut.1);
	/// assert_eq!(&mut 3, tuple_mut.2);
	/// ```
	fn as_mut(&mut self) -> Self::Mut<'_>;
}

/// Tuples that can be joined together. Implemented for tuples of arity 0 to 32.
pub trait JoinableTuple<T: JoinableTuple<Self>>: Tuple {
	/// This tuple joined with another tuple.
	type Join: Tuple;

	/// Joins this tuple with another tuple.
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

/// Tuples that can grow. Implemented for tuples of arity 0 to 31.
pub trait GrowableTuple: Tuple {
	/// This tuple with an extra element `T` appended to it.
	type Append<T>: NonUnitTuple<TruncateTail = Self, Tail = T>;

	/// This tuple with an extra element `T` prepended to it.
	type Prepend<T>: NonUnitTuple<Head = T, TruncateHead = Self>;

	/// Consumes this tuple and appends a value to it, returning a new tuple.
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
	/// ```
	/// # use tupl::GrowableTuple;
	/// let tuple = (2, 3);
	/// let tuple = tuple.prepend(1);
	/// assert_eq!((1, 2, 3), tuple);
	/// ```
	fn prepend<T>(self, value: T) -> Self::Prepend<T>;
}

/// Tuples that contain at least 1 element. Implemented for tuples of arity 1 to 32.
pub trait NonUnitTuple: Tuple {
	/// This tuple with its head truncated.
	type TruncateHead: GrowableTuple<Prepend<Self::Head> = Self>;

	/// This tuple with its tail truncated.
	type TruncateTail: GrowableTuple<Append<Self::Tail> = Self>;

	/// The first element of this tuple.
	type Head;

	/// The last element of this tuple.
	type Tail;

	/// Returns a reference to the head of this tuple.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&1, tuple.head());
	/// ```
	fn head(&self) -> &Self::Head;

	/// Returns a mutable reference to the head of this tuple.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 1, tuple.head_mut());
	/// ```
	fn head_mut(&mut self) -> &mut Self::Head;

	/// Returns a reference to the tail of this tuple.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&3, tuple.tail());
	/// ```
	fn tail(&self) -> &Self::Tail;

	/// Returns a mutable reference to the tail of this tuple.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 3, tuple.tail_mut());
	/// ```
	fn tail_mut(&mut self) -> &mut Self::Tail;

	/// Consumes this tuple and truncates its head from the remaining elements.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let tuple = (1, 2, 3);
	/// let (head, tuple) = tuple.truncate_head();
	/// assert_eq!((1, (2, 3)), (head, tuple));
	/// ```
	fn truncate_head(self) -> (Self::Head, Self::TruncateHead);

	/// Consumes this tuple and truncates its tail from the remaining elements.
	///
	/// ```
	/// # use tupl::NonUnitTuple;
	/// let tuple = (1, 2, 3);
	/// let (tuple, tail) = tuple.truncate_tail();
	/// assert_eq!(((1, 2), 3), (tuple, tail));
	/// ```
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

/// Tuples that contain at least 2 elements. Implemented for tuples of arity 2 to 32.
pub trait PluralTuple:
	NonUnitTuple<
		TruncateHead: NonUnitTuple<TruncateTail = Self::TruncateEnds, Tail = Self::Tail>,
		TruncateTail: NonUnitTuple<Head = Self::Head, TruncateHead = Self::TruncateEnds>,
	>
{
	/// This tuple with its ends truncated.
	type TruncateEnds: GrowableTuple<
			Prepend<Self::Head> = Self::TruncateTail,
			Append<Self::Tail> = Self::TruncateHead,
		>;

	/// Returns a reference to both ends of this tuple.
	///
	/// ```
	/// # use tupl::PluralTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tail) = tuple.ends();
	/// assert_eq!((&1, &4), (head, tail));
	/// ```
	fn ends(&self) -> (&Self::Head, &Self::Tail);

	/// Returns a mutable reference to both ends of this tuple.
	///
	/// ```
	/// # use tupl::PluralTuple;
	/// let mut tuple = (1, 2, 3, 4);
	/// let (head, tail) = tuple.ends_mut();
	/// assert_eq!((&mut 1, &mut 4), (head, tail));
	/// ```
	fn ends_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail);

	/// Consumes this tuple and truncates its ends from the remaining elements.
	///
	/// ```
	/// # use tupl::PluralTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tuple, tail) = tuple.truncate_ends();
	/// assert_eq!((1, (2, 3), 4), (head, tuple, tail));
	/// ```
	fn truncate_ends(self) -> (Self::Head, Self::TruncateEnds, Self::Tail);
}

// Implements all relevant traits.
tupl_macros::impl_traits!();
