use super::*;

/// A function that is called by value and returns a [`Result`](core::result::Result).
pub trait FnResultOnce<T: Tuple>: FnOnce<T, Output = Result<Self::Ok, Self::Err>> {
	/// The `Ok` variant of the [`Result`](core::result::Result) returned by this function.
	type Ok;
	/// The `Err` variant of the [`Result`](core::result::Result) returned by this function.
	type Err;
}

impl<T: Tuple, F: FnOnce<T, Output = Result<Ok, Err>>, Ok, Err> FnResultOnce<T> for F {
	type Ok = Ok;
	type Err = Err;
}

/// A function that is called by mutable reference and returns a [`Result`](core::result::Result).
pub trait FnResultMut<T: Tuple>: FnMut<T> + FnResultOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnResultOnce<T>> FnResultMut<T> for F {}

/// A function that is called by reference and returns a [`Result`](core::result::Result).
pub trait FnResult<T: Tuple>: Fn<T> + FnResultMut<T> {}
impl<T: Tuple, F: Fn<T> + FnResultMut<T>> FnResult<T> for F {}
