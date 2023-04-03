#![allow(confusable_idents)]
#![no_std]

use core::convert::Infallible;

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

/// A trait representing tuples. It is only implemented for tuples of arity 0 to 50.
pub trait Tuple: Sized {
	type Append<T>: Tuple;
	type Prepend<T>: Tuple;

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
}

impl Tuple for () {
	type Append<T> = (T,);
	type Prepend<T> = (T,);
	const ARITY: usize = 0;

	fn append<T>(self, value: T) -> Self::Append<T> {
		(value,)
	}

	fn prepend<T>(self, value: T) -> Self::Prepend<T> {
		(value,)
	}
}

macro_rules! impl_tuple {
	($t0:ident($arity0:literal) $(, $tn:ident($arityn:literal))* $(,)?) => {
		impl_tuple_recursion!($($tn($arityn)),*);

		impl<$t0 $(, $tn)*> Tuple for ($t0, $($tn),*) {
			type Prepend<T> = Infallible;
			type Append<T> = Infallible;
			const ARITY: usize = $arity0;

			#[track_caller]
			fn prepend<T>(self, _: T) -> Self::Prepend<T> {
				panic!("reached maximum tuple arity");
			}

			#[track_caller]
			fn append<T>(self, _: T) -> Self::Append<T> {
				panic!("reached maximum tuple arity");
			}
		}
	};
}

macro_rules! impl_tuple_recursion {
	() => {};
	($t0:ident($arity0:literal) $(, $tn:ident($arityn:literal))* $(,)?) => {
		impl_tuple_recursion!($($tn($arityn)),*);

		#[allow(non_snake_case)]
		impl<$t0 $(, $tn)*> Tuple for ($t0, $($tn),*) {
			type Append<T> = ($t0, $($tn,)* T);
			type Prepend<T> = (T, $t0 $(, $tn)*);
			const ARITY: usize = $arity0;

			fn append<T>(self, value: T) -> Self::Append<T> {
				let ($t0, $($tn,)*) = self;
				($t0, $($tn,)* value)
			}

			fn prepend<T>(self, value: T) -> Self::Prepend<T> {
				let ($t0, $($tn,)*) = self;
				(value, $t0 $(,$tn)*)
			}
		}
	};
}

impl_tuple!(
	ඞ(50),
	Ω(49),
	Ψ(48),
	Χ(47),
	Φ(46),
	Υ(45),
	Τ(44),
	Σ(43),
	Ρ(42),
	Π(41),
	Ο(40),
	Ξ(39),
	Ν(38),
	Μ(37),
	Λ(36),
	Κ(35),
	Ι(34),
	Θ(33),
	Η(32),
	Ζ(31),
	Ε(30),
	Δ(29),
	Γ(28),                     
	Β(27),
	Α(26),
	Z(25),
	Y(24),
	X(23),
	W(22),
	V(21),
	U(20),
	S(19),
	R(18),
	Q(17),
	P(16),
	O(15),
	N(14),
	M(13),
	L(12),
	K(11),
	J(10),
	I(9),
	H(8),
	G(7),
	F(6),
	E(5),
	D(4),
	C(3),
	B(2),
	A(1),
);

impl Tuple for Infallible {
	type Prepend<T> = Infallible;
	type Append<T> = Infallible;
	const ARITY: usize = usize::MAX;

	fn prepend<T>(self, _: T) -> Self::Prepend<T> {
		self
	}

	fn append<T>(self, _: T) -> Self::Append<T> {
		self
	}
}
