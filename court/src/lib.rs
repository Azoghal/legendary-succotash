#![macro_use]
// #![deny(unused)]

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, ItemFn, LitStr, Result, ReturnType, Token};

use quote::{format_ident, quote};

// Attrib is passed the route as a string, client name as an ident
struct TsClientAttrib {
    route: LitStr,
    client_name: Ident,
}

impl Parse for TsClientAttrib {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse a string literal followed by a comma and an identifier
        let route: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let client_name: Ident = input.parse()?;
        Ok(TsClientAttrib { route, client_name })
    }
}

// we use the function body and the attribute arguments to generate code. we return the code as is
// What we actually do is return the token stream as is, but with a test after it,
// and the body of the test does the generation and creation of the files
#[proc_macro_attribute]
pub fn ts_client(attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_input: proc_macro2::TokenStream = input.clone().into();

    let macro_attr = parse_macro_input!(attr as TsClientAttrib);
    println!("Api Route: {}", macro_attr.route.value());
    println!("Api client: {}", macro_attr.client_name);

    let macro_input = parse_macro_input!(input as ItemFn);
    let fn_name = &macro_input.sig.ident;
    let fn_args = &macro_input.sig.inputs;

    println!("Function name: {}", fn_name);

    for input in fn_args {
        match input {
            syn::FnArg::Receiver(_) => println!("Self argument"),
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                let ty = &pat_type.ty;
                println!("Argument name: {:?}", quote!(#pat).to_string());
                println!("Argument type: {:?}", quote!(#ty).to_string());
            }
        }
    }

    let fn_return_type = match &macro_input.sig.output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => quote!(#ty).to_string(),
    };
    println!("Return type: {}", fn_return_type);

    let test_fn = format_ident!("export_bindings_{}", fn_name.to_string().to_lowercase(),);

    let output: proc_macro2::TokenStream = quote! {
        #original_input

        #[cfg(test)]
        #[test]
        fn #test_fn() {
            // TODO make this test append? the templated TS client to the right file?
            panic!("And this is evidence that the files should be generated")
        }
    };

    output.into()
}
