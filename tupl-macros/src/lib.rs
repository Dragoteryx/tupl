//! This crate contains the procedural macros used by the `tupl` crate.

use proc_macro::TokenStream;

mod traits;

/// This macro is used by the `tupl` crate to generate the necessary trait implementations.
#[proc_macro]
pub fn impl_traits(_: TokenStream) -> TokenStream {
	traits::impl_all_traits().into()
}
