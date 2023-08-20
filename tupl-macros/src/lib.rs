use proc_macro::TokenStream;

mod util;

/// This macro is used by the `tupl` crate to generate the necessary trait implementations.
#[proc_macro]
pub fn impl_traits(_: TokenStream) -> TokenStream {
	let idents = util::gen_idents();
	let mut tokens = TokenStream::new();
	for i in 0..=idents.len() {
		tokens.extend(TokenStream::from(util::impl_traits(&idents[..i])));
	}

	tokens
}
