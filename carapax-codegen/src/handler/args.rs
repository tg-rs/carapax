use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Path, Result as SynResult, Token,
};

pub(crate) struct HandlerArgs {
    pub(super) kind: HandlerKind,
}

pub(super) enum HandlerKind {
    Command(Value),
    Predicate(Path),
}

pub(super) enum Value {
    Str(LitStr),
    Path(Path),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(if input.peek(LitStr) {
            Self::Str(input.parse()?)
        } else {
            Self::Path(input.parse()?)
        })
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Str(value) => value.to_tokens(tokens),
            Self::Path(value) => value.to_tokens(tokens),
        }
    }
}

impl Parse for HandlerArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let arg = input.parse::<Ident>()?;
        let kind = match arg.to_string().as_ref() {
            "command" => {
                input.parse::<Token![=]>()?;
                HandlerKind::Command(input.parse()?)
            }
            "predicate" => {
                input.parse::<Token![=]>()?;
                HandlerKind::Predicate(input.parse()?)
            }
            arg => return Err(input.error(format!("unexpected argument: {}", arg))),
        };
        Ok(HandlerArgs { kind })
    }
}
