use proc_macro2::*;
use quote::{format_ident, quote};

const MAX_ARITY: usize = 50;

pub fn gen_idents() -> Vec<Ident> {
	(1usize..=MAX_ARITY)
		.map(|i| format_ident!("T{}", i))
		.collect()
}

pub fn impl_traits(idents: &[Ident]) -> TokenStream {
	let mut tokens = impl_tuple(idents);
	tokens.extend(impl_growable(idents));
	tokens.extend(impl_nonempty(idents));
	tokens.extend(impl_fns(idents));
	tokens
}

pub fn impl_tuple(idents: &[Ident]) -> TokenStream {
	let arity = Literal::usize_suffixed(idents.len());
	quote! {
		impl<#(#idents,)*> private::Sealed for (#(#idents,)*) {}
		impl<#(#idents,)*> Tuple for (#(#idents,)*) {
			const ARITY: usize = #arity;
		}
	}
}

pub fn impl_growable(idents: &[Ident]) -> Option<TokenStream> {
	if idents.len() == MAX_ARITY {
		None
	} else {
		Some(quote! {
			#[allow(non_snake_case)]
			impl<#(#idents,)*> GrowableTuple for (#(#idents,)*) {
				type Append<T> = (#(#idents,)* T,);
				type Prepend<T> = (T, #(#idents,)*);

				fn append<T>(self, value: T) -> Self::Append<T> {
					let (#(#idents,)*) = self;
					(#(#idents,)* value,)
				}

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
			impl<#ident> NonEmptyTuple for (#ident,) {
				type Head = #ident;
				type Tail = #ident;
				type TruncateHead = ();
				type TruncateTail = ();

				fn head(&self) -> &Self::Head {
					&self.0
				}

				fn head_mut(&mut self) -> &mut Self::Head {
					&mut self.0
				}

				fn tail(&self) -> &Self::Tail {
					&self.0
				}

				fn tail_mut(&mut self) -> &mut Self::Tail {
					&mut self.0
				}

				fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
					(self.0, ())
				}

				fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
					((), self.0)
				}
			}
		}),
		[head, rest @ .., tail] => Some(quote! {
			#[allow(non_snake_case)]
			impl<#head, #(#rest,)* #tail> NonEmptyTuple for (#head, #(#rest,)* #tail) {
				type Head = #head;
				type Tail = #tail;
				type TruncateHead = (#(#rest,)* #tail,);
				type TruncateTail = (#head, #(#rest,)*);

				fn head(&self) -> &Self::Head {
					&self.0
				}

				fn head_mut(&mut self) -> &mut Self::Head {
					&mut self.0
				}

				fn tail(&self) -> &Self::Tail {
					let (#head, #(#rest,)* #tail) = self;
					#tail
				}

				fn tail_mut(&mut self) -> &mut Self::Tail {
					let (#head, #(#rest,)* #tail) = self;
					#tail
				}

				fn truncate_head(self) -> (Self::Head, Self::TruncateHead) {
					let (#head, #(#rest,)* #tail) = self;
					(#head, (#(#rest,)* #tail,))
				}

				fn truncate_tail(self) -> (Self::TruncateTail, Self::Tail) {
					let (#head, #(#rest,)* #tail) = self;
					((#head, #(#rest,)*), #tail)
				}
			}
		}),
	}
}

pub fn impl_fns(idents: &[Ident]) -> TokenStream {
	quote! {
		#[allow(non_snake_case)]
		impl<#(#idents,)* F: StdFnOnce(#(#idents,)*) -> Output, Output> FnOnce<(#(#idents,)*)> for F {
			type Output = Output;

			fn call_once(self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[allow(non_snake_case)]
		impl<#(#idents,)* F: StdFnMut(#(#idents,)*) -> Output, Output> FnMut<(#(#idents,)*)> for F {
			fn call_mut(&mut self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}

		#[allow(non_snake_case)]
		impl<#(#idents,)* F: StdFn(#(#idents,)*) -> Output, Output> Fn<(#(#idents,)*)> for F {
			fn call(&self, (#(#idents,)*): (#(#idents,)*)) -> Self::Output {
				self(#(#idents,)*)
			}
		}
	}
}
