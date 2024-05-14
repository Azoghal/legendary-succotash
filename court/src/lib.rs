#![macro_use]
// #![deny(unused)]

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Result, Token};

use proc_macro2;
use quote::{format_ident, quote};

// Attrib keeps the route as a string (unfortunately a need to duplicate this from the api macro for rocket)
// "route/may/include/<...>"
struct TsClientAttrib(Ident);

impl Parse for TsClientAttrib {
    fn parse(input: ParseStream) -> Result<Self> {
        // lookahead one, expect literal? or expect "?
        input.parse().map(TsClientAttrib)
    }
}

// TODO also store the name of the client for it to go into?
// or maybe we can determine this from what file it is in?
// hm
// Input keeps the relevant details from function prototype
// fn fn_name(arg: ArgType, arg_2: ArgType2) -> ResultType { ...
struct TsClientInput {
    fn_name: Ident,
    args: std::vec::Vec<(Ident, Ident)>,
    result_type: Option<Ident>,
}

impl Parse for TsClientInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // TODO actually pay attention to the input
        // Lookahead one, expect fn, then fn name, etc
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![fn]) {
            let _ = input.parse();
            Ok(TsClientInput {
                fn_name: format_ident!("bobis"),
                args: vec![],
                result_type: None,
            })
        } else {
            Err(lookahead.error())
        }
        // Ok(TsClientInput {
        //     fn_name: format_ident!("bobis"),
        //     args: vec![],
        //     result_type: None,
        // })
    }
}

// we use the function body and the attribute arguments to generate code. we return the code as is
// What we actually do is return the token stream as is, but with a test after it,
// and the body of the test does the generation and creation of the files
#[proc_macro_attribute]
pub fn ts_client(attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_input: proc_macro2::TokenStream = input.clone().into();

    // let macro_attr: TsClientAttrib = parse_macro_input!(attr);
    let macro_input: TsClientInput = parse_macro_input!(input);

    // TODO the test name should be derived from the name of the fn
    let test_fn = format_ident!(
        "export_bindings_{}",
        // macro_attr.0.to_string().to_lowercase(),
        macro_input.fn_name.to_string().to_lowercase(),
    );

    let output: proc_macro2::TokenStream = quote! {
        #original_input

        #[cfg(test)]
        #[test]
        fn #test_fn() {

            panic!("And this is evidence that the files should be generated")
        }
    };

    output.into()
}
