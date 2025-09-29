use super::*;
use core::convert::Infallible;
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "alloc")]
use core::any::Any;

pub mod any;

pub trait Visitor<T> {
	type Output;

	fn visit(&mut self, value: T) -> Self::Output;
}

pub trait TupleVisitor<T: Tuple> {
	type Output: Tuple;

	fn visit_tuple(&mut self, tuple: T) -> Self::Output;
}

pub trait FallibleVisitor<T> {
	type Output;
	type Error;

	fn try_visit(&mut self, value: T) -> Result<Self::Output, Self::Error>;
}

pub trait FallibleTupleVisitor<T: Tuple> {
	type Output: Tuple;
	type Error;

	fn try_visit_tuple(&mut self, tuple: T) -> Result<Self::Output, Self::Error>;
}

impl<T, V: Visitor<T>> FallibleVisitor<T> for V {
	type Output = V::Output;
	type Error = Infallible;

	#[inline]
	fn try_visit(&mut self, value: T) -> Result<Self::Output, Self::Error> {
		Ok(self.visit(value))
	}
}

/*impl<T: Tuple, V: TupleVisitor<T>> FallibleTupleVisitor<T> for V {
	type Output = V::Output;
	type Error = Infallible;

	#[inline]
	fn try_visit_tuple(&mut self, tuple: T) -> Result<Self::Output, Self::Error> {
		Ok(self.visit_tuple(tuple))
	}
}*/

pub const fn visit_into<T>() -> VisitInto<T> {
	VisitInto {
		phantom: PhantomData,
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitInto<T> {
	phantom: PhantomData<T>,
}

impl<T, U: Into<T>> Visitor<U> for VisitInto<T> {
	type Output = T;

	#[inline]
	fn visit(&mut self, value: U) -> Self::Output {
		value.into()
	}
}

#[cfg(feature = "alloc")]
pub const fn visit_to_string() -> VisitToString {
	VisitToString(())
}

#[repr(transparent)]
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitToString(());

#[cfg(feature = "alloc")]
impl<T: ToString> Visitor<T> for VisitToString {
	type Output = String;

	#[inline]
	fn visit(&mut self, value: T) -> Self::Output {
		value.to_string()
	}
}

pub const fn visit_some() -> VisitSome {
	VisitSome(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitSome(());
impl<T> Visitor<T> for VisitSome {
	type Output = Option<T>;

	#[inline]
	fn visit(&mut self, value: T) -> Self::Output {
		Some(value)
	}
}
