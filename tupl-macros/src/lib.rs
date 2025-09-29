#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

const MAX_ARITY: usize = 32;

mod impls;

/// This macro is used by the `tupl` crate to generate the necessary trait implementations.
#[proc_macro]
pub fn impl_traits(_: TokenStream) -> TokenStream {
	impls::impl_all_traits().into()
}
