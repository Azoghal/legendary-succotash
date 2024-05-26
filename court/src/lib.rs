#![macro_use]
// #![deny(unused)]
use itertools::Itertools;

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

// TODO all type names must be typscriptinated.
// - also, for example i32 -> number rather than rTypes....

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

    // TODO actually whack these into a vec
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

    // TODO what if the return type is Result? we're only interested in the
    let fn_return_type = match &macro_input.sig.output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => quote!(#ty).to_string(),
    };
    println!("Return type: {}", fn_return_type);

    let test_fn = format_ident!("export_bindings_{}", fn_name.to_string().to_lowercase());

    let ts_fn_body = format_ts_api(
        &fn_name.to_string(),
        vec![("bob", "i32")],
        &fn_return_type,
        &macro_attr.route.value(),
    );

    let client_name: String = macro_attr.client_name.to_string();

    let output: proc_macro2::TokenStream = quote! {
        #original_input

        #[cfg(test)]
        #[test]
        fn #test_fn() {
            use std::fs::{File, OpenOptions};
            use std::io::{self, Seek, SeekFrom, Write};

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}.ts", #client_name))
                .unwrap();

            // Check if the file is empty (indicating it was just created)
            let file_metadata = file.metadata().unwrap();
            if file_metadata.len() == 0 {
                // File is empty, write the header first
                // TODO although maybe we'll need all sorts of imports... :(
                writeln!(
                    file,
                    "import {{ BaseClient }} from \"./baseClient\"\n
                    import * as rTypes from \"./exampleModels\""
                )
                .unwrap();
            }

            // Append the content
            writeln!(file, "{}", #ts_fn_body).unwrap();
        }
    };

    output.into()
}

// TODO check that we are using the best api route version as template
fn format_ts_api(
    fn_name: &str,
    arg_types: Vec<(&str, &str)>,
    return_type: &str,
    route: &str,
) -> String {
    let arg_types_str = arg_types
        .iter()
        .map(|c| format!("{}: rTypes.{}", c.0, c.1))
        .join(",");

    format!(
        "
        async function {fn_name}(baseClient: BaseClient, {arg_types_str}): Promise<rTypes.{return_type}> {{
            const route = baseClient.baseUrl + \"{route}\";
            console.log(bob);
            return fetch(route)
                .then((response) => {{ return response.json()}})
                .then((json) => json as rTypes.{return_type});
        }}
    "
    )
}
