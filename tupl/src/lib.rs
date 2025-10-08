#![allow(async_fn_in_trait, non_snake_case, clippy::unused_unit)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::convert::Infallible;
use core::iter::FusedIterator;
use core::iter::{Empty, empty};
use core::mem::ManuallyDrop;

/// Function related traits.
pub mod fns;
use fns::*;

/// Visit heterogeneous tuples.
pub mod visit;
use visit::*;

// Sealed trait.
mod seal {
	pub trait Sealed: Sized {}
}

/// All tuples. Implemented for tuples of arity 0 to 32.
pub trait Tuple: seal::Sealed {
	/// The [arity](https://en.wikipedia.org/wiki/Arity) (or length) of this tuple.
	const ARITY: usize;

	/// Whether this tuple is the unit tuple or not.
	const IS_UNIT: bool;

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

	/// Visits every element of this tuple using the provided visitor.
	///
	/// ```
	/// # use tupl::Tuple;
	/// # use tupl::visit::Visitor;
	/// struct IncrementVisitor;
	///
	/// impl Visitor<&mut i32> for IncrementVisitor {
	/// 	type Output = ();
	///
	/// 	fn visit(&mut self, value: &mut i32) {
	/// 		*value += 1;
	/// 	}
	/// }
	///
	/// impl Visitor<&mut f64> for IncrementVisitor {
	/// 	type Output = ();
	///
	/// 	fn visit(&mut self, value: &mut f64) {
	/// 		*value += 1.0;
	/// 	}
	/// }
	///
	/// let mut tuple = (1, 2.0, 3);
	/// let mut visitor = IncrementVisitor;
	/// tuple.as_mut().visit(&mut visitor);
	/// assert_eq!((2, 3.0, 4), tuple);
	/// ```
	#[inline]
	fn visit<V>(self, visitor: &mut V) -> V::Output
	where
		V: TupleVisitor<Self>,
	{
		visitor.visit_tuple(self)
	}

	#[inline]
	fn try_visit<V>(self, visitor: &mut V) -> Result<V::Output, V::Error>
	where
		V: FallibleTupleVisitor<Self>,
	{
		visitor.try_visit_tuple(self)
	}
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
	type Append<T>: NonEmptyTuple<TruncateTail = Self, Tail = T>;

	/// This tuple with an extra element `T` prepended to it.
	type Prepend<T>: NonEmptyTuple<Head = T, TruncateHead = Self>;

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

/// Tuples that are not empty. Implemented for tuples of arity 1 to 32.
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
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&1, tuple.head());
	/// ```
	fn head(&self) -> &Self::Head;

	/// Returns a mutable reference to the head of this tuple.
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 1, tuple.head_mut());
	/// ```
	fn head_mut(&mut self) -> &mut Self::Head;

	/// Returns a reference to the tail of this tuple.
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&3, tuple.tail());
	/// ```
	fn tail(&self) -> &Self::Tail;

	/// Returns a mutable reference to the tail of this tuple.
	///
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 3, tuple.tail_mut());
	/// ```
	fn tail_mut(&mut self) -> &mut Self::Tail;

	/// Consumes this tuple and truncates its head from the remaining elements.
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
	/// ```
	/// # use tupl::NonEmptyTuple;
	/// let tuple = (1, 2, 3);
	/// let (tuple, tail) = tuple.truncate_tail();
	/// assert_eq!(((1, 2), 3), (tuple, tail));
	/// ```
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

/// Tuples that not unary nor empty. Implemented for tuples of arity 2 to 32.
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
	/// ```
	/// # use tupl::NonUnaryTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tail) = tuple.head_tail();
	/// assert_eq!((&1, &4), (head, tail));
	/// ```
	fn head_tail(&self) -> (&Self::Head, &Self::Tail);

	/// Returns a mutable reference to the head and tail of this tuple.
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
	/// ```
	/// # use tupl::NonUnaryTuple;
	/// let tuple = (1, 2, 3, 4);
	/// let (head, tuple, tail) = tuple.truncate_head_tail();
	/// assert_eq!((1, (2, 3), 4), (head, tuple, tail));
	/// ```
	fn truncate_head_tail(self) -> (Self::Head, Self::TruncateHeadTail, Self::Tail);
}

/// Tuples that contain elements of a single type. Implemented for tuples of arity 0 to 32.
pub trait HomogeneousTuple: Tuple {
	/// An iterator over the elements of this tuple.
	type IntoIter: Iterator<Item = Self::Item>;

	/// An iterator over the elements of this tuple, yielding references.
	type Iter<'t>: Iterator<Item = &'t Self::Item>
	where
		Self: 't;

	/// An iterator over the elements of this tuple, yielding mutable references.
	type IterMut<'t>: Iterator<Item = &'t mut Self::Item>
	where
		Self: 't;

	/// The type of the elements of this tuple.
	type Item;

	/// Returns a reference to the element at the given index.
	///
	/// ```
	/// # use tupl::HomogeneousTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(Some(&1), tuple.get(0));
	/// assert_eq!(Some(&2), tuple.get(1));
	/// assert_eq!(Some(&3), tuple.get(2));
	/// assert_eq!(None, tuple.get(3));
	/// ```
	fn get(&self, index: usize) -> Option<&Self::Item>;

	/// Returns a mutable reference to the element at the given index.
	///
	/// ```
	/// # use tupl::HomogeneousTuple;
	/// let mut tuple = (1, 2, 3);
	/// *tuple.get_mut(0).unwrap() += 1;
	/// *tuple.get_mut(1).unwrap() += 2;
	/// *tuple.get_mut(2).unwrap() += 3;
	/// assert_eq!((2, 4, 6), tuple);
	/// ```
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	/// Returns the value at the given index, consuming the tuple.
	///
	/// ```
	/// # use tupl::HomogeneousTuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(Ok(2), tuple.into_inner(1));
	/// ```
	fn into_inner(self, index: usize) -> Result<Self::Item, Self>;

	fn into_iter(self) -> Self::IntoIter;

	fn iter(&self) -> Self::Iter<'_>;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

// Implements all relevant traits.
tupl_macros::impl_traits!();

pub struct TupleIter<T: HomogeneousTuple> {
	tuple: ManuallyDrop<T>,
	index: usize,
}

impl<T: HomogeneousTuple> FusedIterator for TupleIter<T> {}
impl<T: HomogeneousTuple> ExactSizeIterator for TupleIter<T> {}
impl<T: HomogeneousTuple> Iterator for TupleIter<T> {
	type Item = T::Item;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			if let Some(ptr) = self.tuple.get(self.index) {
				let item = core::ptr::read(ptr);
				self.index += 1;
				Some(item)
			} else {
				None
			}
		}
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let remaining = T::ARITY - self.index;
		(remaining, Some(remaining))
	}
}

impl<T: HomogeneousTuple> Drop for TupleIter<T> {
	fn drop(&mut self) {
		unsafe {
			while let Some(ptr) = self.tuple.get_mut(self.index) {
				core::ptr::drop_in_place(ptr);
				self.index += 1;
			}
		}
	}
}
