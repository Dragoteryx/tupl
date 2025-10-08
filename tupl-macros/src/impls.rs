use proc_macro2::*;
use quote::{format_ident, quote};

pub fn impl_all_traits() -> TokenStream {
	let mut tokens = TokenStream::new();
	let idents = (1..=crate::MAX_ARITY)
		.map(|i| format_ident!("T{i}"))
		.collect::<Vec<_>>();

	for i in 0..=idents.len() {
		tokens.extend(impl_traits(&idents[..i]));
	}

	tokens
}

pub fn impl_traits(idents: &[Ident]) -> TokenStream {
	let mut tokens = impl_tuple(idents);
	tokens.extend(impl_joinable(idents));
	tokens.extend(impl_growable(idents));
	tokens.extend(impl_nonempty(idents));
	tokens.extend(impl_homogeneous(idents));
	tokens.extend(impl_visitor(idents));
	tokens.extend(impl_fns(idents));
	tokens
}

pub fn impl_tuple(idents: &[Ident]) -> TokenStream {
	let is_unit = idents.is_empty();
	let arity = idents.len();

	quote! {
		#[automatically_derived]
		impl<#(#idents,)*> seal::Sealed for (#(#idents,)*) {}

		#[automatically_derived]
		impl<#(#idents,)*> Tuple for (#(#idents,)*) {
			type Ref<'t> = (#(&'t #idents,)*) where Self: 't;
			type Mut<'t> = (#(&'t mut #idents,)*) where Self: 't;
			const ARITY: usize = #arity;
			const IS_UNIT: bool = #is_unit;

			#[inline]
			fn as_ref(&self) -> Self::Ref<'_> {
				let (#(#idents,)*) = self;
				(#(#idents,)*)
			}

			#[inline]
			fn as_mut(&mut self) -> Self::Mut<'_> {
				let (#(#idents,)*) = self;
				(#(#idents,)*)
			}
		}
	}
}

pub fn impl_joinable(idents: &[Ident]) -> TokenStream {
	let mut tokens = TokenStream::new();
	for i in 0..=idents.len() {
		let (left, right) = idents.split_at(i);
		tokens.extend(quote! {
			#[automatically_derived]
			impl<#(#left,)* #(#right,)*> JoinableTuple<(#(#right,)*)> for (#(#left,)*) {
				type Join = (#(#left,)* #(#right,)*);

				#[inline]
				fn join(self, other: (#(#right,)*)) -> Self::Join {
					let (#(#left,)*) = self;
					let (#(#right,)*) = other;
					(#(#left,)* #(#right,)*)
				}
			}
		});
	}

	tokens
}

pub fn impl_growable(idents: &[Ident]) -> Option<TokenStream> {
	if idents.len() == crate::MAX_ARITY {
		None
	} else {
		Some(quote! {
			#[automatically_derived]
			impl<#(#idents,)*> GrowableTuple for (#(#idents,)*) {
				type Append<T> = (#(#idents,)* T,);
				type Prepend<T> = (T, #(#idents,)*);

				#[inline]
				fn append<T>(self, value: T) -> Self::Append<T> {
					let (#(#idents,)*) = self;
					(#(#idents,)* value,)
				}

				#[inline]
				fn prepend<T>(self, value: T) -> Self::Prepend<T> {
					let (#(#idents,)*) = self;
					(value, #(#idents,)*)
				}
			}
		})
	}
}

pub fn impl_nonempty(idents: &[Ident]) -> Option<TokenStream> {
	match idents {
		[] => None,
		[ident] => Some(quote! {
			#[automatically_derived]
			impl<#ident> NonEmptyTuple for (#ident,) {
				type Head = #ident;
				type Tail = #ident;
				type TruncateHead = ();
				type TruncateTail = ();

				#[inline]
				fn head(&self) -> &Self::Head {
					&self.0
				}

				#[inline]
				fn head_mut(&mut self) -> &mut Self::Head {
					&mut self.0
				}

				#[inline]
				fn tail(&self) -> &Self::Tail {
					&self.0
				}

				#[inline]
				fn tail_mut(&mut self) -> &mut Self::Tail {
					&mut self.0
				}

				#[inline]
				fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
					(self.0, ())
				}

				#[inline]
				fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
					((), self.0)
				}
			}
		}),
		[head, rest @ .., tail] => {
			let tail_idx = Literal::usize_unsuffixed(idents.len() - 1);
			Some(quote! {
				#[automatically_derived]
				impl<#head, #(#rest,)* #tail> NonEmptyTuple for (#head, #(#rest,)* #tail) {
					type Head = #head;
					type Tail = #tail;
					type TruncateHead = (#(#rest,)* #tail,);
					type TruncateTail = (#head, #(#rest,)*);

					#[inline]
					fn head(&self) -> &Self::Head {
						&self.0
					}

					#[inline]
					fn head_mut(&mut self) -> &mut Self::Head {
						&mut self.0
					}

					#[inline]
					fn tail(&self) -> &Self::Tail {
						&self.#tail_idx
					}

					#[inline]
					fn tail_mut(&mut self) -> &mut Self::Tail {
						&mut self.#tail_idx
					}

					#[inline]
					fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
						let (#head, #(#rest,)* #tail) = self;
						(#head, (#(#rest,)* #tail,))
					}

					#[inline]
					fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
						let (#head, #(#rest,)* #tail) = self;
						((#head, #(#rest,)*), #tail)
					}
				}

				#[automatically_derived]
				impl<#head, #(#rest,)* #tail> NonUnaryTuple for (#head, #(#rest,)* #tail) {
					type TruncateHeadTail = (#(#rest,)*);

					#[inline]
					fn head_tail(&self) -> (&Self::Head, &Self::Tail) {
						(&self.0, &self.#tail_idx)
					}

					#[inline]
					fn head_tail_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail) {
						(&mut self.0, &mut self.#tail_idx)
					}

					#[inline]
					fn truncate_head_tail(self) -> (Self::Head, Self::TruncateHeadTail, Self::Tail) {
						let (#head, #(#rest,)* #tail) = self;
						(#head, (#(#rest,)*), #tail)
					}
				}
			})
		}
	}
}

pub fn impl_homogeneous(idents: &[Ident]) -> TokenStream {
	if idents.is_empty() {
		quote! {
			impl HomogeneousTuple for () {
				type IterMut<'t> = Empty<&'t mut Infallible> where Self: 't;
				type Iter<'t> = Empty<&'t Infallible> where Self: 't;
				type IntoIter = Empty<Infallible>;
				type Item = Infallible;

				fn get(&self, _index: usize) -> Option<&Self::Item> {
					None
				}

				fn get_mut(&mut self, _index: usize) -> Option<&mut Self::Item> {
					None
				}

				fn into_inner(self, _index: usize) -> Result<Self::Item, Self> {
					Err(self)
				}

				fn into_iter(self) -> Self::IntoIter {
					empty()
				}

				fn iter(&self) -> Self::Iter<'_> {
					empty()
				}

				fn iter_mut(&mut self) -> Self::IterMut<'_> {
					empty()
				}
			}
		}
	} else {
		let ident = format_ident!("T");
		let idents = idents.iter().map(|_| &ident);
		let indexes = (0..idents.len())
			.map(Literal::usize_unsuffixed)
			.collect::<Vec<_>>();

		quote! {
			impl<#ident> HomogeneousTuple for (#(#idents,)*) {
				type IterMut<'t> = TupleIter<Self::Mut<'t>> where Self: 't;
				type Iter<'t> = TupleIter<Self::Ref<'t>> where Self: 't;
				type IntoIter = TupleIter<Self>;
				type Item = #ident;

				#[inline]
				fn get(&self, index: usize) -> Option<&Self::Item> {
					match index {
						#(#indexes => Some(&self.#indexes),)*
						_ => None,
					}
				}

				#[inline]
				fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
					match index {
						#(#indexes => Some(&mut self.#indexes),)*
						_ => None,
					}
				}

				#[inline]
				fn into_inner(self, index: usize) -> Result<Self::Item, Self> {
					match index {
						#(#indexes => Ok(self.#indexes),)*
						_ => Err(self),
					}
				}

				#[inline]
				fn into_iter(self) -> Self::IntoIter {
					TupleIter {
						tuple: ManuallyDrop::new(self),
						index: 0,
					}
				}

				#[inline]
				fn iter(&self) -> Self::Iter<'_> {
					self.as_ref().into_iter()
				}

				#[inline]
				fn iter_mut(&mut self) -> Self::IterMut<'_> {
					self.as_mut().into_iter()
				}
			}
		}
	}
}

pub fn impl_visitor(idents: &[Ident]) -> TokenStream {
	let mut tokens = quote! {
		#[automatically_derived]
		impl<V, #(#idents,)*> TupleVisitor<(#(#idents,)*)> for V
		where
			#(V: Visitor<#idents>,)*
		{
			type Output = (#(<V as Visitor<#idents>>::Output,)*);

			#[inline]
			fn visit_tuple(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				(#(self.visit(#idents),)*)
			}
		}
	};

	let tokens2 = match idents {
		[] => quote! {
			#[automatically_derived]
			impl<V> FallibleTupleVisitor<()> for V {
				type Output = ();
				type Error = Infallible;

				#[inline]
				fn try_visit_tuple(&mut self, (): ()) -> Result<Self::Output, Self::Error> {
					Ok(())
				}
			}
		},
		[head, rest @ ..] => quote! {
			#[automatically_derived]
			impl<V, #head, #(#rest,)*> FallibleTupleVisitor<(#head, #(#rest,)*)> for V
			where
				V: FallibleVisitor<#head>,
				#(V: FallibleVisitor<#rest>,)*
				#(<V as FallibleVisitor<#head>>::Error: From<<V as FallibleVisitor<#rest>>::Error>,)*
			{
				type Output = (<V as FallibleVisitor<#head>>::Output, #(<V as FallibleVisitor<#rest>>::Output,)*);
				type Error = <V as FallibleVisitor<#head>>::Error;

				#[inline]
				fn try_visit_tuple(&mut self, (#head, #(#rest,)*): (#head, #(#rest,)*)) -> Result<Self::Output, Self::Error> {
					Ok((
						self.try_visit(#head)?,
						#(self.try_visit(#rest)?,)*
					))
				}
			}
		},
	};

	tokens.extend(tokens2);
	tokens
}

pub fn impl_fns(idents: &[Ident]) -> TokenStream {
	quote! {
		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::FnOnce(#(#idents,)*) -> Output, Output> FnOnce<(#(#idents,)*)> for F {
			type Output = Output;

			#[inline]
			fn call_once(self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::FnMut(#(#idents,)*) -> Output, Output> FnMut<(#(#idents,)*)> for F {

			#[inline]
			fn call_mut(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::Fn(#(#idents,)*) -> Output, Output> Fn<(#(#idents,)*)> for F {

			#[inline]
			fn call(&self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::AsyncFnOnce(#(#idents,)*) -> Output, Output> AsyncFnOnce<(#(#idents,)*)> for F {
			type Output = Output;

			#[inline]
			async fn async_call_once(self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}

		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::AsyncFnMut(#(#idents,)*) -> Output, Output> AsyncFnMut<(#(#idents,)*)> for F {

			#[inline]
			async fn async_call_mut(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}

		#[automatically_derived]
		impl<#(#idents,)* F: core::ops::AsyncFn(#(#idents,)*) -> Output, Output> AsyncFn<(#(#idents,)*)> for F {

			#[inline]
			async fn async_call(&self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}
	}
}
