use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Path, Result as SynResult, Token,
};

pub(crate) struct HandlerArgs {
    pub(super) kind: HandlerKind,
}

pub(super) enum HandlerKind {
    Command(LitStr),
    Predicate(Path),
}

impl Parse for HandlerArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let arg = input.parse::<Ident>()?;
        let kind = match arg.to_string().as_ref() {
            "command" => {
                input.parse::<Token![=]>()?;
                let name = input.parse::<LitStr>()?;
                if !name.value().starts_with('/') {
                    return Err(input.error("command name must start with '/'"));
                }
                HandlerKind::Command(name)
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
