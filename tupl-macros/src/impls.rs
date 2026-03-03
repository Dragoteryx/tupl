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
	tokens.extend(impl_fns(idents));
	tokens
}

pub fn impl_tuple(idents: &[Ident]) -> TokenStream {
	let unit = idents.is_empty();
	let arity = idents.len();

	quote! {
		#[automatically_derived]
		impl<#(#idents,)*> seal::Sealed for (#(#idents,)*) {}

		#[automatically_derived]
		impl<#(#idents,)*> Tuple for (#(#idents,)*) {
			type Ref<'t> = (#(&'t #idents,)*) where Self: 't;
			type Mut<'t> = (#(&'t mut #idents,)*) where Self: 't;
			const ARITY: usize = #arity;
			const UNIT: bool = #unit;

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
			impl<#ident> NonUnitTuple for (#ident,) {
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
				impl<#head, #(#rest,)* #tail> NonUnitTuple for (#head, #(#rest,)* #tail) {
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
				impl<#head, #(#rest,)* #tail> PluralTuple for (#head, #(#rest,)* #tail) {
					type TruncateEnds = (#(#rest,)*);

					#[inline]
					fn ends(&self) -> (&Self::Head, &Self::Tail) {
						(&self.0, &self.#tail_idx)
					}

					#[inline]
					fn ends_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail) {
						(&mut self.0, &mut self.#tail_idx)
					}

					#[inline]
					fn truncate_ends(self) -> (Self::Head, Self::TruncateEnds, Self::Tail) {
						let (#head, #(#rest,)* #tail) = self;
						(#head, (#(#rest,)*), #tail)
					}
				}
			})
		}
	}
}

pub fn impl_fns(idents: &[Ident]) -> TokenStream {
	quote! {
		#[automatically_derived]
		impl<F: core::ops::FnOnce(#(#idents,)*) -> O, #(#idents,)* O> fns::FnOnce<(#(#idents,)*)> for F {
			type Output = O;

			#[inline]
			fn call_once(self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<F: core::ops::FnMut(#(#idents,)*) -> O, #(#idents,)* O> fns::FnMut<(#(#idents,)*)> for F {

			#[inline]
			fn call_mut(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<F: core::ops::Fn(#(#idents,)*) -> O, #(#idents,)* O> fns::Fn<(#(#idents,)*)> for F {

			#[inline]
			fn call(&self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[automatically_derived]
		impl<F: core::ops::AsyncFnOnce(#(#idents,)*) -> O, #(#idents,)* O> fns::AsyncFnOnce<(#(#idents,)*)> for F {
			type Output = O;

			#[inline]
			async fn async_call_once(self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}

		#[automatically_derived]
		impl<F: core::ops::AsyncFnMut(#(#idents,)*) -> O, #(#idents,)* O> fns::AsyncFnMut<(#(#idents,)*)> for F {

			#[inline]
			async fn async_call_mut(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}

		#[automatically_derived]
		impl<F: core::ops::AsyncFn(#(#idents,)*) -> O, #(#idents,)* O> fns::AsyncFn<(#(#idents,)*)> for F {

			#[inline]
			async fn async_call(&self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*).await
			}
		}
	}
}
