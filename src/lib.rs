#![allow(confusable_idents)]
#![no_std]

//! A `#![no_std]` crate for handling Rust tuples using traits.

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

impl private::Sealed for () {}
impl Tuple for () {
	const ARITY: usize = 0;
}

impl GrowableTuple for () {
	type Append<T> = (T,);
	type Prepend<T> = (T,);

	fn append<T>(self, value: T) -> Self::Append<T> {
		(value,)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value,)
	}
}

impl<A> private::Sealed for (A,) {}
impl<A> Tuple for (A,) {
	const ARITY: usize = 1;
}

impl<A> GrowableTuple for (A,) {
	type Append<T> = (A, T);
	type Prepend<T> = (T, A);

	fn append<T>(self, value: T) -> Self::Append<T> {
		(self.0, value)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value, self.0)
	}
}

impl<A> NonEmptyTuple for (A,) {
	type Head = A;
	type Tail = A;
	type TruncateHead = ();
	type TruncateTail = ();

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

	fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
		(self.0, ())
	}

	fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
		((), self.0)
	}
}

macro_rules! impl_tuple {
	($t0:ident => $tn:ident; $t1:ident | $arity0:literal $(, $tx:ident | $arityn:literal)* $(,)?) => {
		impl_tuple_recursion!($t0 => $tn; $($tx | $arityn),*);

		impl<A, $t1 $(, $tx)*, $tn> private::Sealed for (A, $t1, $($tx,)* $tn) {}
		impl<A, $t1 $(, $tx)*, $tn> Tuple for (A, $t1, $($tx,)* $tn) {
			const ARITY: usize = $arity0;
		}

		#[allow(non_snake_case)]
		impl<A, $t1 $(, $tx)*, $tn> NonEmptyTuple for (A, $t1, $($tx,)* $tn) {
			type Head = A;
			type Tail = $tn;
			type TruncateHead = ($t1, $($tx,)* $tn);
			type TruncateTail = (A, $t1, $($tx,)*);

			fn head(&self) -> &Self::Head {
				&self.0
			}

			fn head_mut(&mut self) -> &mut Self::Head {
				&mut self.0
			}

			fn tail(&self) -> &Self::Tail {
				let (.., $tn) = self;
				$tn
			}

			fn tail_mut(&mut self) -> &mut Self::Tail {
				let (.., $tn) = self;
				$tn
			}

			fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
				let ($t0, $t1, $($tx,)* $tn) = self;
				($t0, ($t1, $($tx,)* $tn))
			}

			fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
				let ($t0, $t1, $($tx,)* $tn) = self;
				(($t0, $t1 $(,$tx)*), $tn)
			}
		}
	};
}

macro_rules! impl_tuple_recursion {
	($t0:ident => $tn:ident $(;)?) => {
		impl<$t0, $tn> private::Sealed for ($t0, $tn) {}
		impl<$t0, $tn> Tuple for ($t0, $tn) {
			const ARITY: usize = 2;
		}

		#[allow(non_snake_case)]
		impl<$t0, $tn> GrowableTuple for ($t0, $tn) {
			type Append<ඞ> = ($t0, $tn, ඞ);
			type Prepend<ඞ> = (ඞ, $t0, $tn);

			fn append<ඞ>(self, value: ඞ) -> Self::Append<ඞ> {
				(self.0, self.1, value)
			}

			fn prepend<ඞ>(self, value: ඞ) -> Self::Prepend<ඞ> {
				(value, self.0, self.1)
			}
		}

		#[allow(non_snake_case)]
		impl<$t0, $tn> NonEmptyTuple for ($t0, $tn) {
			type Head = $t0;
			type Tail = $tn;
			type TruncateHead = ($tn,);
			type TruncateTail = ($t0,);

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

			fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
				(self.0, (self.1,))
			}

			fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
				((self.0,), self.1)
			}
		}
	};
	($t0:ident => $tn:ident; $t1:ident | $arity0:literal $(, $tx:ident | $arityn:literal)* $(,)?) => {
		impl_tuple_recursion!($t0 => $tn; $($tx | $arityn),*);

		impl<$t0, $t1 $(, $tx)*, $tn> private::Sealed for ($t0, $t1, $($tx,)* $tn) {}
		impl<$t0, $t1 $(, $tx)*, $tn> Tuple for ($t0, $t1, $($tx,)* $tn) {
			const ARITY: usize = $arity0;
		}

		#[allow(non_snake_case)]
		impl<$t0, $t1 $(, $tx)*, $tn> GrowableTuple for ($t0, $t1, $($tx,)* $tn) {
			type Append<ඞ> = ($t0, $t1, $($tx,)* $tn, ඞ);
			type Prepend<ඞ> = (ඞ, $t0, $t1, $($tx,)* $tn);

			fn append<ඞ>(self, value: ඞ) -> Self::Append<ඞ> {
				let ($t0, $t1, $($tx,)* $tn) = self;
				($t0, $t1, $($tx,)* $tn, value)
			}

			fn prepend<ඞ>(self, value: ඞ) -> Self::Prepend<ඞ> {
				let ($t0, $t1, $($tx,)* $tn) = self;
				(value, $t0, $t1, $($tx,)* $tn)
			}
		}

		#[allow(non_snake_case)]
		impl<$t0, $t1 $(, $tx)*, $tn> NonEmptyTuple for ($t0, $t1, $($tx,)* $tn) {
			type Head = $t0;
			type Tail = $tn;
			type TruncateHead = ($t1, $($tx,)* $tn);
			type TruncateTail = ($t0, $t1, $($tx,)*);

			fn head(&self) -> &Self::Head {
				&self.0
			}

			fn head_mut(&mut self) -> &mut Self::Head {
				&mut self.0
			}

			fn tail(&self) -> &Self::Tail {
				let (.., $tn) = self;
				$tn
			}

			fn tail_mut(&mut self) -> &mut Self::Tail {
				let (.., $tn) = self;
				$tn
			}

			fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
				let ($t0, $t1, $($tx,)* $tn) = self;
				($t0, ($t1, $($tx,)* $tn))
			}

			fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
				let ($t0, $t1, $($tx,)* $tn) = self;
				(($t0, $t1 $(,$tx)*), $tn)
			}
		}
	};
}

impl_tuple!(
	A => Ω;

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
