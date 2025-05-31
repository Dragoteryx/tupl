use super::*;

/// Functions that are called by value and return an [`Option`].
pub trait FnOptionOnce<T: Tuple>: FnOnce<T, Output = Option<Self::Some>> {
	/// The `Some` variant of the [`Option`] returned by this function.
	type Some;
}

impl<T: Tuple, F: FnOnce<T, Output = Option<Some>>, Some> FnOptionOnce<T> for F {
	type Some = Some;
}

/// Functions that can be called by mutable reference and return an [`Option`].
pub trait FnOptionMut<T: Tuple>: FnMut<T> + FnOptionOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnOptionOnce<T>> FnOptionMut<T> for F {}

/// Functions that can be called by reference and return an [`Option`].
pub trait FnOption<T: Tuple>: Fn<T> + FnOptionMut<T> {}
impl<T: Tuple, F: Fn<T> + FnOptionMut<T>> FnOption<T> for F {}

/// Async functions that are called by value and return an [`Option`].
pub trait AsyncFnOptionOnce<T: Tuple>: AsyncFnOnce<T, Output = Option<Self::Some>> {
	/// The `Some` variant of the [`Option`] returned by this async function.
	type Some;
}

impl<T: Tuple, F: AsyncFnOnce<T, Output = Option<Some>>, Some> AsyncFnOptionOnce<T> for F {
	type Some = Some;
}

/// Async functions that can be called by mutable reference and return an [`Option`].
pub trait AsyncFnOptionMut<T: Tuple>: AsyncFnMut<T> + AsyncFnOptionOnce<T> {}
impl<T: Tuple, F: AsyncFnMut<T> + AsyncFnOptionOnce<T>> AsyncFnOptionMut<T> for F {}

/// Async functions that can be called by reference and return an [`Option`].
pub trait AsyncFnOption<T: Tuple>: AsyncFn<T> + AsyncFnOptionMut<T> {}
impl<T: Tuple, F: AsyncFn<T> + AsyncFnOptionMut<T>> AsyncFnOption<T> for F {}
