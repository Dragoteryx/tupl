use super::*;

/// Functions that are called by value and return a [`Result`].
pub trait FnResultOnce<T: Tuple>: FnOnce<T, Output = Result<Self::Ok, Self::Err>> {
	/// The `Ok` variant of the [`Result`] returned by this function.
	type Ok;
	/// The `Err` variant of the [`Result`] returned by this function.
	type Err;
}

impl<T: Tuple, F: FnOnce<T, Output = Result<Ok, Err>>, Ok, Err> FnResultOnce<T> for F {
	type Ok = Ok;
	type Err = Err;
}

/// Functions that can be called by mutable reference and return a [`Result`].
pub trait FnResultMut<T: Tuple>: FnMut<T> + FnResultOnce<T> {}
impl<T: Tuple, F: FnMut<T> + FnResultOnce<T>> FnResultMut<T> for F {}

/// Functions that can be called by reference and return a [`Result`].
pub trait FnResult<T: Tuple>: Fn<T> + FnResultMut<T> {}
impl<T: Tuple, F: Fn<T> + FnResultMut<T>> FnResult<T> for F {}

