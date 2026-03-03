use super::*;

/// Functions that return an [`Option`].
pub mod option;

/// Functions that return a [`Result`].
pub mod result;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurryFirst<T, F> {
	func: F,
	arg: T,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurryLast<T, F> {
	func: F,
	arg: T,
}

/// Functions that are called by value.
pub trait FnOnce<T: Tuple> {
	/// The return type of this function.
	type Output;

	/// Call the function by value.
	fn call_once(self, args: T) -> Self::Output;

	/// Curry the first argument of this function.
	#[inline]
	fn curry_first(self, arg: T::Head) -> CurryFirst<T::Head, Self>
	where
		T: NonUnitTuple,
		Self: Sized,
	{
		CurryFirst { func: self, arg }
	}

	/// Curry the last argument of this function.
	#[inline]
	fn curry_last(self, arg: T::Tail) -> CurryLast<T::Tail, Self>
	where
		T: NonUnitTuple,
		Self: Sized,
	{
		CurryLast { func: self, arg }
	}
}

/// Functions that can be called by mutable reference.
pub trait FnMut<T: Tuple>: FnOnce<T> {
	/// Call the function by mutable reference.
	fn call_mut(&mut self, args: T) -> Self::Output;
}

/// Functions that can be called by reference.
pub trait Fn<T: Tuple>: FnMut<T> {
	/// Call the function by reference.
	fn call(&self, args: T) -> Self::Output;
}

/// Async functions that are called by value.
pub trait AsyncFnOnce<T: Tuple> {
	/// The return type of this async function.
	type Output;

	/// Call the async function by value.
	async fn async_call_once(self, args: T) -> Self::Output;

	/// Curry the first argument of this async function.
	#[inline]
	fn curry_first(self, arg: T::Head) -> CurryFirst<T::Head, Self>
	where
		T: NonUnitTuple,
		Self: Sized,
	{
		CurryFirst { func: self, arg }
	}

	/// Curry the last argument of this async function.
	#[inline]
	fn curry_last(self, arg: T::Tail) -> CurryLast<T::Tail, Self>
	where
		T: NonUnitTuple,
		Self: Sized,
	{
		CurryLast { func: self, arg }
	}
}

/// Async functions that are called by mutable reference.
pub trait AsyncFnMut<T: Tuple>: AsyncFnOnce<T> {
	/// Call the async function by mutable reference.
	async fn async_call_mut(&mut self, args: T) -> Self::Output;
}

/// Async functions that are called by reference.
pub trait AsyncFn<T: Tuple>: AsyncFnMut<T> {
	/// Call the async function by reference.
	async fn async_call(&self, args: T) -> Self::Output;
}

impl<T, U: GrowableTuple, F: FnOnce<U::Prepend<T>>> FnOnce<U> for CurryFirst<T, F> {
	type Output = F::Output;

	#[inline]
	fn call_once(self, args: U) -> Self::Output {
		self.func.call_once(args.prepend(self.arg))
	}
}

impl<T: Clone, U: GrowableTuple, F: FnMut<U::Prepend<T>>> FnMut<U> for CurryFirst<T, F> {
	#[inline]
	fn call_mut(&mut self, args: U) -> Self::Output {
		self.func.call_mut(args.prepend(self.arg.clone()))
	}
}

impl<T: Clone, U: GrowableTuple, F: Fn<U::Prepend<T>>> Fn<U> for CurryFirst<T, F> {
	#[inline]
	fn call(&self, args: U) -> Self::Output {
		self.func.call(args.prepend(self.arg.clone()))
	}
}

impl<T, U: GrowableTuple, F: FnOnce<U::Append<T>>> FnOnce<U> for CurryLast<T, F> {
	type Output = F::Output;

	#[inline]
	fn call_once(self, args: U) -> Self::Output {
		self.func.call_once(args.append(self.arg))
	}
}

impl<T: Clone, U: GrowableTuple, F: FnMut<U::Append<T>>> FnMut<U> for CurryLast<T, F> {
	#[inline]
	fn call_mut(&mut self, args: U) -> Self::Output {
		self.func.call_mut(args.append(self.arg.clone()))
	}
}

impl<T: Clone, U: GrowableTuple, F: Fn<U::Append<T>>> Fn<U> for CurryLast<T, F> {
	#[inline]
	fn call(&self, args: U) -> Self::Output {
		self.func.call(args.append(self.arg.clone()))
	}
}

impl<T, U: GrowableTuple, F: AsyncFnOnce<U::Prepend<T>>> AsyncFnOnce<U> for CurryFirst<T, F> {
	type Output = F::Output;

	#[inline]
	async fn async_call_once(self, args: U) -> Self::Output {
		self.func.async_call_once(args.prepend(self.arg)).await
	}
}

impl<T: Clone, U: GrowableTuple, F: AsyncFnMut<U::Prepend<T>>> AsyncFnMut<U> for CurryFirst<T, F> {
	#[inline]
	async fn async_call_mut(&mut self, args: U) -> Self::Output {
		self.func.async_call_mut(args.prepend(self.arg.clone())).await
	}
}

impl<T: Clone, U: GrowableTuple, F: AsyncFn<U::Prepend<T>>> AsyncFn<U> for CurryFirst<T, F> {
	#[inline]
	async fn async_call(&self, args: U) -> Self::Output {
		self.func.async_call(args.prepend(self.arg.clone())).await
	}
}

impl<T, U: GrowableTuple, F: AsyncFnOnce<U::Append<T>>> AsyncFnOnce<U> for CurryLast<T, F> {
	type Output = F::Output;

	#[inline]
	async fn async_call_once(self, args: U) -> Self::Output {
		self.func.async_call_once(args.append(self.arg)).await
	}
}

impl<T: Clone, U: GrowableTuple, F: AsyncFnMut<U::Append<T>>> AsyncFnMut<U> for CurryLast<T, F> {
	#[inline]
	async fn async_call_mut(&mut self, args: U) -> Self::Output {
		self.func.async_call_mut(args.append(self.arg.clone())).await
	}
}

impl<T: Clone, U: GrowableTuple, F: AsyncFn<U::Append<T>>> AsyncFn<U> for CurryLast<T, F> {
	#[inline]
	async fn async_call(&self, args: U) -> Self::Output {
		self.func.async_call(args.append(self.arg.clone())).await
	}
}