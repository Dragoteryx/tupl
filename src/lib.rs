#![allow(confusable_idents)]
#![no_std]

//! A small library for handling Rust tuples using traits.

use core::convert::Infallible;
use core::mem;

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
	/// The first element of this tuple.
	type Head;

	/// The last element of this tuple.
	type Tail;

	/// This tuple with an extra element `T` appended to it.
	type Append<T>: Tuple;

	/// This tuple with an extra element `T` prepended to it.
	type Prepend<T>: Tuple;

	/// This tuple with its head truncated.
	type TruncateHead: Tuple;
	
	/// This tuple with its tail truncated.
	type TruncateTail: Tuple;

	/// The arity or length of this tuple type.
	const ARITY: usize;

	/// Returns a reference to the head of this tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&1, tuple.head());
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn head(&self) -> &Self::Head;

	/// Returns a mutable reference to the head of this tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 1, tuple.head_mut());
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn head_mut(&mut self) -> &mut Self::Head;

	/// Replaces the head of this tuple with a new value, returning the old one.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// let head = tuple.replace_head(-1);
	/// assert_eq!(-1, tuple.0);
	/// assert_eq!(1, head);
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn replace_head(&mut self, head: Self::Head) -> Self::Head {
		mem::replace(self.head_mut(), head)
	}

	/// Takes the head of this tuple and replaces it with its default value, returning the current value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// let head = tuple.take_head();
	/// assert_eq!(0, tuple.0);
	/// assert_eq!(1, head);
	/// ```
	/// 
	/// # Panics
	///
	/// Calling this function on an empty tuple will cause a panic.
	fn take_head(&mut self) -> Self::Head
	where
		Self::Head: Default
	{
		mem::take(self.head_mut())
	}

	/// Replaces the tail of this tuple with a new value, returning the old one.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// let tail = tuple.replace_tail(4);
	/// assert_eq!(4, tuple.2);
	/// assert_eq!(3, tail);
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn replace_tail(&mut self, tail: Self::Tail) -> Self::Tail {
		mem::replace(self.tail_mut(), tail)
	}

	/// Takes the tail of this tuple and replaces it with its default value, returning the current value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// let tail = tuple.take_tail();
	/// assert_eq!(0, tuple.2);
	/// assert_eq!(3, tail);
	/// ```
	/// 
	/// # Panics
	///
	/// Calling this function on an empty tuple will cause a panic.
	fn take_tail(&mut self) -> Self::Tail
	where
		Self::Tail: Default
	{
		mem::take(self.tail_mut())
	}

	/// Returns a reference to the tail of this tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// assert_eq!(&3, tuple.tail());
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn tail(&self) -> &Self::Tail;

	/// Returns a mutable reference to the tail of this tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let mut tuple = (1, 2, 3);
	/// assert_eq!(&mut 3, tuple.tail_mut());
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn tail_mut(&mut self) -> &mut Self::Tail;

	/// Consumes this tuple and appends a value to it, returning a new tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2);
	/// let tuple = tuple.append(3);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on a tuple of arity 50 or greater will cause a panic.
	fn append<T>(self, value: T) -> Self::Append<T>;

	/// Consumes this tuple and prepends a value to it, returning a new tuple.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (2, 3);
	/// let tuple = tuple.prepend(1);
	/// assert_eq!(tuple, (1, 2, 3));
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on a tuple of arity 50 or greater will cause a panic.
	fn prepend<T>(self, value: T) -> Self::Prepend<T>;

	/// Consumes this tuple and truncates its head from the remaining elements.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_head();
	/// assert_eq!(tuple, (1, (2, 3)));
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn truncate_head(self) -> (Self::Head, Self::TruncateHead);

	/// Consumes this tuple and truncates its tail from the remaining elements.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use tupl::Tuple;
	/// let tuple = (1, 2, 3);
	/// let tuple = tuple.truncate_tail();
	/// assert_eq!(tuple, ((1, 2), 3));
	/// ```
	/// 
	/// # Panics
	/// 
	/// Calling this function on an empty tuple will cause a panic.
	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail);
}

impl private::Sealed for Infallible {}
impl Tuple for Infallible {
	type Head = Infallible;
	type Tail = Infallible;
	type Append<T> = Infallible;
	type Prepend<T> = Infallible;
	type TruncateHead = Infallible;
	type TruncateTail = Infallible;
	const ARITY: usize = usize::MAX;

	fn head(&self) -> &Self::Head {
		self
	}

	fn head_mut(&mut self) -> &mut Self::Head {
		self
	}

	fn tail(&self) -> &Self::Tail {
		self
	}

	fn tail_mut(&mut self) -> &mut Self::Tail {
		self
	}

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
	type Head = Infallible;
	type Tail = Infallible;
	type Append<T> = (T,);
	type Prepend<T> = (T,);
	type TruncateHead = Infallible;
	type TruncateTail = Infallible;
	const ARITY: usize = 0;

	fn head(&self) -> &Self::Head {
		panic!("tried to get the head of an empty tuple")
	}

	fn head_mut(&mut self) -> &mut Self::Head {
		panic!("tried to get the head of an empty tuple")
	}

	fn tail(&self) -> &Self::Tail {
		panic!("tried to get the tail of an empty tuple")
	}

	fn tail_mut(&mut self) -> &mut Self::Tail {
		panic!("tried to get the tail of an empty tuple")
	}

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
	type Head = A;
	type Tail = A;
	type Append<T> = (A, T);
	type Prepend<T> = (T, A);
	type TruncateHead = ();
	type TruncateTail = ();
	const ARITY: usize = 1;

	fn head(&self) -> &Self::Head {
		&self.0
	}

	fn head_mut(&mut self) -> &mut Self::Head {
		&mut self.0
	}

	fn tail(&self) -> &Self::Tail {
		&self.0
	}

	fn tail_mut(&mut self) -> &mut Self::Tail {
		&mut self.0
	}

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
	type Head = A;
	type Tail = Ω;
	type Append<T> = (A, Ω, T);
	type Prepend<T> = (T, A, Ω);
	type TruncateHead = (Ω,);
	type TruncateTail = (A,);
	const ARITY: usize = 2;

	fn head(&self) -> &Self::Head {
		&self.0
	}

	fn head_mut(&mut self) -> &mut Self::Head {
		&mut self.0
	}

	fn tail(&self) -> &Self::Tail {
		&self.1
	}

	fn tail_mut(&mut self) -> &mut Self::Tail {
		&mut self.1
	}

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
			type Head = A;
			type Tail = Ω;
			type Append<ඞ> = Infallible;
			type Prepend<ඞ> = Infallible;
			type TruncateHead = ($t0, $($tn,)* Ω);
			type TruncateTail = (A, $t0, $($tn,)*);
			const ARITY: usize = $arity0;

			fn head(&self) -> &Self::Head {
				&self.0
			}

			fn head_mut(&mut self) -> &mut Self::Head {
				&mut self.0
			}

			fn tail(&self) -> &Self::Tail {
				let (.., tail) = self;
				tail
			}

			fn tail_mut(&mut self) -> &mut Self::Tail {
				let (.., tail) = self;
				tail
			}

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
			type Head = A;
			type Tail = Ω;
			type Append<ඞ> = (A, $t0, $($tn,)* Ω, ඞ);
			type Prepend<ඞ> = (ඞ, A, $t0, $($tn,)* Ω);
			type TruncateHead = ($t0, $($tn,)* Ω);
			type TruncateTail = (A, $t0, $($tn,)*);
			const ARITY: usize = $arity0;

			fn head(&self) -> &Self::Head {
				&self.0
			}

			fn head_mut(&mut self) -> &mut Self::Head {
				&mut self.0
			}

			fn tail(&self) -> &Self::Tail {
				let (.., tail) = self;
				tail
			}

			fn tail_mut(&mut self) -> &mut Self::Tail {
				let (.., tail) = self;
				tail
			}

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
