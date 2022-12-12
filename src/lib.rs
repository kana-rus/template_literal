use proc_macro::TokenStream;
mod internals;

#[proc_macro]
pub fn tmp(literal: TokenStream) -> TokenStream {
    match internals::tmp(literal.into()) {
        Ok(result) => result,
        Err(error) => error.into_compile_error(),
    }.into()
}