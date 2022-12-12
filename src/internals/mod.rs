use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::{parse2, Result, LitStr, Expr};

trait Interpret<T> {
    fn interpret(self) -> T;
}
trait Build {
    fn build(self) -> TokenStream;
}

// mod tmp;
pub(super) fn tmp(input: TokenStream) -> Result<TokenStream> {
    let literal = parse2::<LitStr>(input.clone())?.value();
    let mut interpolations = vec![];

    if literal.len() < 3 {
        return if &literal == "${" {
            Err(syn::Error::new(Span::call_site(), "Not closed interpolution"))
        } else {
            Ok(input)
        }
    }

    for i in 0..=literal.len()-3 {
        if &literal[i..=i+1] == "${" {
            let Some(close) = &literal[i+2..].find('}')
                else {return Err(syn::Error::new(Span::call_site(), "Not closed interpolution"))};
            interpolations.push((i, i+2+*close))
        }
    }

    let n_interpolutions = interpolations.len();
    if n_interpolutions == 0 {return Ok(input)}

    let (mut format, mut args) = (String::new(), TokenStream::new());
    let comma = quote!(,);
    for (i, (open, close)) in interpolations.into_iter().enumerate() {
        if i == 0 {
            format += &literal[0..open]
        }

        if open + 2 == close {return Err(syn::Error::new(Span::call_site(), "Empty interpolution"))}
        comma.to_tokens(&mut args);
        syn::parse_str::<Expr>(&literal[open+2..close])?.to_tokens(&mut args);
        format += "{}";
        
        if i == n_interpolutions-1 {
            format += &literal[close+1..]
        }
    }

    Ok(quote!(
        format!(#format #args)
    ))
}