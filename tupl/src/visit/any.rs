use super::*;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[inline]
#[cfg(feature = "alloc")]
pub const fn visit_any() -> VisitAny {
	VisitAny(())
}

#[repr(transparent)]
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAny(());

#[cfg(feature = "alloc")]
impl<T: 'static> Visitor<T> for VisitAny {
	type Output = Box<dyn Any>;

	#[inline]
	fn visit(&mut self, value: T) -> Self::Output {
		Box::new(value)
	}
}

#[inline]
#[cfg(feature = "alloc")]
pub const fn visit_any_send() -> VisitAnySend {
	VisitAnySend(())
}

#[repr(transparent)]
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySend(());

#[cfg(feature = "alloc")]
impl<T: Send + 'static> Visitor<T> for VisitAnySend {
	type Output = Box<dyn Any + Send>;

	#[inline]
	fn visit(&mut self, value: T) -> Self::Output {
		Box::new(value)
	}
}

#[inline]
#[cfg(feature = "alloc")]
pub const fn visit_any_send_sync() -> VisitAnySendSync {
	VisitAnySendSync(())
}

#[repr(transparent)]
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySendSync(());

#[cfg(feature = "alloc")]
impl<T: Send + Sync + 'static> Visitor<T> for VisitAnySendSync {
	type Output = Box<dyn Any + Send + Sync>;

	#[inline]
	fn visit(&mut self, value: T) -> Self::Output {
		Box::new(value)
	}
}

#[inline]
pub const fn visit_any_ref() -> VisitAnyRef {
	VisitAnyRef(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnyRef(());
impl<'a, T: 'static> Visitor<&'a T> for VisitAnyRef {
	type Output = &'a dyn Any;

	#[inline]
	fn visit(&mut self, value: &'a T) -> Self::Output {
		value
	}
}

#[inline]
pub const fn visit_any_send_ref() -> VisitAnySendRef {
	VisitAnySendRef(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySendRef(());
impl<'a, T: Send + 'static> Visitor<&'a T> for VisitAnySendRef {
	type Output = &'a (dyn Any + Send);

	#[inline]
	fn visit(&mut self, value: &'a T) -> Self::Output {
		value
	}
}

#[inline]
pub const fn visit_any_send_sync_ref() -> VisitAnySendSyncRef {
	VisitAnySendSyncRef(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySendSyncRef(());
impl<'a, T: Send + Sync + 'static> Visitor<&'a T> for VisitAnySendSyncRef {
	type Output = &'a (dyn Any + Send + Sync);

	#[inline]
	fn visit(&mut self, value: &'a T) -> Self::Output {
		value
	}
}

#[inline]
pub const fn visit_any_mut() -> VisitAnyMut {
	VisitAnyMut(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnyMut(());
impl<'a, T: 'static> Visitor<&'a mut T> for VisitAnyMut {
	type Output = &'a mut dyn Any;

	#[inline]
	fn visit(&mut self, value: &'a mut T) -> Self::Output {
		value
	}
}

#[inline]
pub const fn visit_any_send_mut() -> VisitAnySendMut {
	VisitAnySendMut(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySendMut(());
impl<'a, T: Send + 'static> Visitor<&'a mut T> for VisitAnySendMut {
	type Output = &'a mut (dyn Any + Send);

	#[inline]
	fn visit(&mut self, value: &'a mut T) -> Self::Output {
		value
	}
}

#[inline]
pub const fn visit_any_send_sync_mut() -> VisitAnySendSyncMut {
	VisitAnySendSyncMut(())
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisitAnySendSyncMut(());
impl<'a, T: Send + Sync + 'static> Visitor<&'a mut T> for VisitAnySendSyncMut {
	type Output = &'a mut (dyn Any + Send + Sync);

	#[inline]
	fn visit(&mut self, value: &'a mut T) -> Self::Output {
		value
	}
}
