extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod dialogue;
mod handler;

#[proc_macro_attribute]
pub fn dialogue(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as dialogue::DialogueMeta);
    TokenStream::from(dialogue::build(input))
}

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
