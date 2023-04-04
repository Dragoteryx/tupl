#![allow(confusable_idents)]
#![no_std]

//! A small library for handling Rust tuples using traits.

use core::convert::Infallible;
mod private {
	pub trait Sealed: Sized {}
}

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

/// A trait representing tuples. It is only implemented for tuples of arity 0 to 50.\
/// This trait is **sealed** and as such not meant to be implemented.
pub trait Tuple: private::Sealed {
	type Append<T>: Tuple;
	type Prepend<T>: Tuple;
	type Head;
	type TruncateHead: Tuple;
	type Tail;
	type TruncateTail: Tuple;

	/// The arity or length of this tuple type.
	const ARITY: usize;

	/// Consumes this tuple and appends a value to it, returning a new tuple.
	/// # Examples
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2);
	/// let tuple = tuple.append(3);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	/// # Panics
	/// Calling this function on a tuple of arity 50 or greater will cause a panic.
	fn append<T>(self, value: T) -> Self::Append<T>;

	/// Consumes this tuple and prepends a value to it, returning a new tuple.
	/// # Examples
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (2, 3);
	/// let tuple = tuple.prepend(1);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	/// # Panics
	/// Calling this function on a tuple of arity 50 or greater will cause a panic.
	fn prepend<T>(self, value: T) -> Self::Prepend<T>;

	/// Consumes this tuple and truncates the first element from the remaining elements.
	/// # Examples
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_head();
	/// assert_eq!(tuple, (1, (2, 3)));
	/// ```
	/// # Panics
	/// Calling this function on a tuple of arity 0 will cause a panic.
	fn truncate_head(self) -> (Self::Head, Self::TruncateHead);

	/// Consumes this tuple and truncates the last element from the remaining elements.
	/// # Examples
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_tail();
	/// assert_eq!(tuple, ((1, 2), 3));
	/// ```
	/// # Panics
	/// Calling this function on a tuple of arity 0 will cause a panic.
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

impl private::Sealed for Infallible {}
impl Tuple for Infallible {
	type Append<T> = Infallible;
	type Prepend<T> = Infallible;
	type Head = Infallible;
	type TruncateHead = Infallible;
	type Tail = Infallible;
	type TruncateTail = Infallible;
	const ARITY: usize = usize::MAX;

	fn prepend<T>(self, _: T) -> Self::Prepend<T> {
		self
	}

	fn append<T>(self, _: T) -> Self::Append<T> {
		self
	}

	fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
		match self {}
	}

	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
		match self {}
	}
}

impl private::Sealed for () {}
impl Tuple for () {
	type Append<T> = (T,);
	type Prepend<T> = (T,);
	type Head = Infallible;
	type TruncateHead = Infallible;
	type Tail = Infallible;
	type TruncateTail = Infallible;
	const ARITY: usize = 0;

	fn append<T>(self, value: T) -> Self::Append<T> {
		(value,)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value,)
	}

	fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
		panic!("tried to truncate empty tuple")
	}

	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
		panic!("tried to truncate empty tuple")
	}
}

impl<A> private::Sealed for (A,) {}
impl<A> Tuple for (A,) {
	type Append<T> = (A, T);
	type Prepend<T> = (T, A);
	type Head = A;
	type TruncateHead = ();
	type Tail = A;
	type TruncateTail = ();
	const ARITY: usize = 1;

	fn append<T>(self, value: T) -> Self::Append<T> {
		(self.0, value,)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value, self.0)
	}

	fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
		(self.0, ())
	}

	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
		((), self.0)
	}
}

impl<A, Ω> private::Sealed for (A, Ω) {}
impl<A, Ω> Tuple for (A, Ω) {
	type Append<T> = (A, Ω, T);
	type Prepend<T> = (T, A, Ω);
	type Head = A;
	type TruncateHead = (Ω,);
	type Tail = Ω;
	type TruncateTail = (A,);
	const ARITY: usize = 2;

	fn append<T>(self, value: T) -> Self::Append<T> {
		(self.0, self.1, value,)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value, self.0, self.1)
	}

	fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
		(self.0, (self.1,))
	}

	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
		((self.0,), self.1)
	}
}

macro_rules! impl_tuple {
	($t0:ident | $arity0:literal $(, $tn:ident | $arityn:literal)* $(,)?) => {
		impl_tuple_recursion!($($tn | $arityn),*);

		impl<A, $t0 $(, $tn)*, Ω> private::Sealed for (A, $t0, $($tn),*, Ω) {}
		impl<A, $t0 $(, $tn)*, Ω> Tuple for (A, $t0, $($tn),*, Ω) {
			type Append<ඞ> = Infallible;
			type Prepend<ඞ> = Infallible;
			type Head = A;
			type TruncateHead = ($t0, $($tn,)* Ω);
			type Tail = Ω;
			type TruncateTail = (A, $t0, $($tn,)*);
			const ARITY: usize = $arity0;

			#[track_caller]
			fn prepend<ඞ>(self, _: ඞ) -> Self::Prepend<ඞ> {
				panic!("reached maximum tuple arity");
			}

			#[track_caller]
			fn append<ඞ>(self, _: ඞ) -> Self::Append<ඞ> {
				panic!("reached maximum tuple arity");
			}

			#[allow(non_snake_case)]
			fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
				let (head, $t0, $($tn,)* tail) = self;
				(head, ($t0, $($tn,)* tail))
			}

			#[allow(non_snake_case)]
			fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
				let (head, $t0, $($tn,)* tail) = self;
				((head, $t0 $(,$tn)*), tail)
			}
		}
	};
}

macro_rules! impl_tuple_recursion {
	() => {};
	($t0:ident | $arity0:literal $(, $tn:ident | $arityn:literal)* $(,)?) => {
		impl_tuple_recursion!($($tn | $arityn),*);

		impl<A, $t0 $(, $tn)*, Ω> private::Sealed for (A, $t0, $($tn,)* Ω) {}
		impl<A, $t0 $(, $tn)*, Ω> Tuple for (A, $t0, $($tn,)* Ω) {
			type Append<ඞ> = (A, $t0, $($tn,)* Ω, ඞ);
			type Prepend<ඞ> = (ඞ, A, $t0, $($tn,)* Ω);
			type Head = A;
			type TruncateHead = ($t0, $($tn,)* Ω);
			type Tail = Ω;
			type TruncateTail = (A, $t0, $($tn,)*);
			const ARITY: usize = $arity0;

			#[allow(non_snake_case)]
			fn append<ඞ>(self, value: ඞ) -> Self::Append<ඞ> {
				let (head, $t0, $($tn,)* tail) = self;
				(head, $t0, $($tn,)* tail, value)
			}

			#[allow(non_snake_case)]
			fn prepend<ඞ>(self, value: ඞ) -> Self::Prepend<ඞ> {
				let (head, $t0, $($tn,)* tail) = self;
				(value, head, $t0, $($tn,)* tail)
			}

			#[allow(non_snake_case)]
			fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
				let (head, $t0, $($tn,)* tail) = self;
				(head, ($t0, $($tn,)* tail))
			}

			#[allow(non_snake_case)]
			fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
				let (head, $t0, $($tn,)* tail) = self;
				((head, $t0 $(,$tn)*), tail)
			}
		}
	};
}

impl_tuple!(
	B | 50,
	C | 49,
	D | 48,
	E | 47,
	F | 46,
	G | 45,
	H | 44,
	I | 43,
	J | 42,
	K | 41,
	L | 40,
	M | 39,
	N | 38,
	O | 37,
	P | 36,
	Q | 35,
	R | 34,
	S | 33,
	T | 32,
	U | 31,
	V | 30,
	W | 29,
	X | 28,
	Y | 27,
	Z | 26,
	Α | 25,
	Β | 24,
	Γ | 23,
	Δ | 22,
	Ε | 21,
	Ζ | 20,
	Η | 19,
	Θ | 18,
	Ι | 17,
	Κ | 16,
	Λ | 15,
	Μ | 14,
	Ν | 13,
	Ξ | 12,
	Ο | 11,
	Π | 10,
	Ρ | 9,
	Σ | 8,
	Τ | 7,
	Υ | 6,
	Φ | 5,
	Χ | 4,
	Ψ | 3,
);
