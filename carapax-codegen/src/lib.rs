extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod handler;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as handler::HandlerMeta);
    let args = if args.is_empty() {
        None
    } else {
        Some(parse_macro_input!(args as handler::HandlerArgs))
    };
    TokenStream::from(handler::build(input, args))
}
