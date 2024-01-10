use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_include_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Config);
    let input = input.param;
    let expanded = quote! {
        let contents = include_str!(#input);
        println!("{contents}");

    };
    proc_macro::TokenStream::from(expanded)
}

struct Config {
    param: String,
}

impl Parse for Config {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input
            .parse::<Option<syn::LitStr>>()?
            .map(|param| Self {
                param: param.value(),
            })
            .ok_or(syn::Error::new(
                input.span(),
                "generate_include_str! takes 1 argument",
            ))
    }
}
