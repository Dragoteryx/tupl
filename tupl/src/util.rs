use super::*;
use core::iter::Chain;
use core::iter::{Empty, empty};
use core::iter::{Once, once};

pub trait TupleInto<T>: Tuple {
	type Iter: Iterator<Item = T>;

	fn tuple_into(self) -> Self::Iter;
}

impl<T> TupleInto<T> for () {
	type Iter = Empty<T>;

	#[inline]
	fn tuple_into(self) -> Self::Iter {
		empty()
	}
}

impl<T, U: NonUnitTuple<TruncateTail: TupleInto<T>, Tail: Into<T>>> TupleInto<T> for U {
	type Iter = Chain<<U::TruncateTail as TupleInto<T>>::Iter, Once<T>>;

	#[inline]
	fn tuple_into(self) -> Self::Iter {
		let (tuple, tail) = self.truncate_tail();
		tuple.tuple_into().chain(once(tail.into()))
	}
}
