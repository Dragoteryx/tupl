use super::*;

/// A function that is called by value and returns an [`Option`](core::option::Option).
pub trait FnOptionOnce<T: Tuple>: FnOnce<T, Output = Option<Self::Some>> {
	/// The `Some` variant of the [`Option`](core::option::Option) returned by this function.
	type Some;
}

impl<T: Tuple, F: FnOnce<T, Output = Option<Some>>, Some> FnOptionOnce<T> for F {
	type Some = Some;
}

/// A function that is called by mutable reference and returns an [`Option`](core::option::Option).
pub trait FnOptionMut<T: Tuple>: FnMut<T> + FnOptionOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnOptionOnce<T>> FnOptionMut<T> for F {}

/// A function that is called by reference and returns an [`Option`](core::option::Option).
pub trait FnOption<T: Tuple>: Fn<T> + FnOptionMut<T> {}
impl<T: Tuple, F: Fn<T> + FnOptionMut<T>> FnOption<T> for F {}
