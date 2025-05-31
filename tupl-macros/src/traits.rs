use proc_macro2::*;
use quote::{format_ident, quote};

pub const MAX_ARITY: usize = 32;

pub fn impl_all_traits() -> TokenStream {
	let mut tokens = TokenStream::new();
	let idents = (1..=MAX_ARITY)
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
	tokens.extend(impl_indexable(idents));
	tokens.extend(impl_fns(idents));
	tokens
}

pub fn impl_tuple(idents: &[Ident]) -> TokenStream {
	let arity = Literal::usize_suffixed(idents.len());
	quote! {
		#[automatically_derived]
		impl<#(#idents,)*> seal::Sealed for (#(#idents,)*) {}

		#[automatically_derived]
		impl<#(#idents,)*> DynTuple for (#(#idents,)*) {
			#[inline]
			fn arity(&self) -> usize {
				#arity
			}
		}

		#[automatically_derived]
		impl<#(#idents,)*> Tuple for (#(#idents,)*) {
			const ARITY: usize = #arity;
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
	if idents.len() == MAX_ARITY {
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

pub fn impl_indexable(idents: &[Ident]) -> TokenStream {
	let mut tokens = TokenStream::new();
	for (i, ident) in idents.iter().enumerate() {
		let index = Literal::usize_unsuffixed(i);
		tokens.extend(quote! {
			#[automatically_derived]
			impl<#(#idents,)*> IndexableTuple<#index> for (#(#idents,)*) {
				type Value = #ident;

				#[inline]
				fn index_ref(&self) -> &Self::Value {
					&self.#index
				}

				#[inline]
				fn index_mut(&mut self) -> &mut Self::Value {
					&mut self.#index
				}

				#[inline]
				fn into_index(self) -> Self::Value {
					self.#index
				}
			}
		});
	}

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
